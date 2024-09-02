use std::{
    cmp::min,
    io::{self, stdout, Write},
    thread,
    time::{Duration, Instant},
    option::Option,
};

use crossterm::{
    cursor::{self, MoveDown, MoveTo, MoveToColumn}, 
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers}, 
    style::Print, 
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    execute, queue, 
};

use super::TermUnit;

use crate::{
    config::Config, 
    mode::{SourceDictionary, SourceFile, WordSource}, 
    punctuation::Punct,
    textbox::Text
};

#[derive(Debug)]
enum Engine {
    Ready,
    Stop,
    Run,
    ShowPunct,
    TooNarrow
}


pub struct BoxInfo {
    pub left_padding : TermUnit,
    pub top_padding : TermUnit,
    pub width : TermUnit,
    pub size : (TermUnit, TermUnit),
}

impl BoxInfo {
    pub fn centered(box_width : TermUnit, size : (TermUnit, TermUnit)) 
    -> Result<BoxInfo, ()> {
        if size.0 < box_width + 1 || size.1 < 9 {return Err(())}
        let left_padding = (size.0 - box_width) / 2;
        let top_padding = (size.1 - 4) / 2;

        Ok(BoxInfo {
            left_padding,
            top_padding,
            width : box_width,
            size,
        })
    }
}

impl Default for BoxInfo {
    fn default() -> Self {
        let size = crossterm::terminal::size().unwrap();

        BoxInfo {
            left_padding : 0,
            top_padding : 0,
            width : 0,
            size,
        }
    }
}

pub struct Mecano { 
    input_offset : TermUnit,
    typed_word : String,
    textbox : Text,
    box_info : BoxInfo,
    width : TermUnit,
    lines_to_show : TermUnit,
    engine : Engine,
    punct : Option<Punct>,
}

impl Mecano {

    pub fn play(config : Config) -> io::Result<()> {
        let fps = config.get_rate();
        let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
        let mut delta;
        let mut chrono = Instant::now();

        let mut engine = Mecano::new(config)?;
        engine.draw()?;

        while !engine.is_ended() {

            while let Ok(true) = poll(Duration::ZERO) {
                if !engine.is_running() && !engine.is_too_narrow() {
                    engine.run();
                }
                let keep_going = engine.event_read()?;
                if !keep_going {
                    return Ok(());
                }
            }

            if engine.is_running() {
                engine.update_time(frame_duration)?;
            }

            delta = frame_duration - min(frame_duration, chrono.elapsed());
            thread::sleep(delta);
            chrono = Instant::now();
        }

        engine.draw()?;
        loop {
            while let Ok(true) = poll(Duration::ZERO) {
                if !engine.event_read()? {
                    return Ok(());
                }
            }

            delta = frame_duration - min(frame_duration, chrono.elapsed());
            thread::sleep(delta);
            chrono = Instant::now();
        }
    }

    fn new(config : Config) -> io::Result<Mecano> {
        let words_source = Mecano::word_source(&config);

        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let box_info = BoxInfo::centered(
            config.get_width(), 
            crossterm::terminal::size().unwrap_or((0, 0))
        ).unwrap_or_default();

        let buffer = Text::new(
            words_source,
            config.get_config_text_box().clone(),
            config.get_max_time(),
            (std::cmp::min::<TermUnit>( box_info.size.0 / 2 - 2,
                config.get_lenght()),
                box_info.width)
        );

        let state : Mecano = Mecano {
            typed_word : String::new(),
            input_offset : box_info.left_padding,
            textbox: buffer, 
            box_info, 
            engine : Engine::Ready,
            width : config.get_width(),
            lines_to_show : config.get_lenght(),
            punct : None,
        };

        return Ok(state);
    }


    fn run(&mut self) { self.engine = Engine::Run }

    fn is_running(&mut self) -> bool {
        return match self.engine { 
            Engine::Run => true,
            _ => false,
            } 
    }

    fn stop(&mut self) { self.engine = Engine::Stop }

    fn is_stopped(&self) -> bool {
        return match self.engine { 
            Engine::Stop => true,
            _ => false,
            } 
    }

    fn end(&mut self) { self.engine = Engine::ShowPunct; }

    fn is_ended(&mut self) -> bool { 
        return match self.engine { 
            Engine::ShowPunct => true,
            _ => false,
            } 
    }

    fn too_narrow(&mut self) { self.engine = Engine::TooNarrow; }

    fn is_too_narrow(&self) -> bool {
        return match self.engine { 
            Engine::TooNarrow => true,
            _ => false,
            } 
    }

    fn is_ready(&self) -> bool {
        return match self.engine { 
            Engine::Ready => true,
            _ => false,
            } 
    }

    fn update_time(&mut self, elapsed : Duration) -> io::Result<()> {
        let keep_going = self.textbox.update_time(elapsed);
        if !keep_going {
            self.end();
        }

        match self.engine {
            Engine::Ready | Engine::Run | Engine::Stop => { self.print_time()? }
            _ => ()
        }

        return Ok(());
    }

    fn draw(&mut self) -> io::Result<()> {

        queue!(stdout(), Clear(ClearType::All))?;
        let real_size = crossterm::terminal::size().unwrap();

        if let Ok(box_info) = BoxInfo::centered(self.width, real_size) {
            self.box_info = box_info;
            self.textbox.set_size((self.box_info.width, self.lines_to_show()));
            self.textbox.set_column(self.box_info.left_padding);

            if self.is_ended() { 
                self.engine = Engine::ShowPunct;
            } 
        } else if !self.is_ended() {
            self.too_narrow();
        }

        match self.engine {
            Engine::Run | Engine::Stop => {
                self.draw_playing()?;
            }
            Engine::Ready => {
                self.draw_ready()?;
            }
            Engine::ShowPunct => {
                self.draw_punct()?;
            }
            Engine::TooNarrow => {
                self.draw_too_narrow()?;
            }
        }

        return Ok(());
    }

    fn word_source(config : &Config) -> Box<dyn WordSource> {
        return match config.get_mode().as_str() {
            "file" => Box::new(SourceFile::from_config(&config)),
            "dictionary" => Box::new(SourceDictionary::from_config(&config)),
            _ => panic!()

        }
    }

    // TO DO : Add controls information for size
    fn draw_ready(&mut self) -> io::Result<()> {
        return self.draw_playing();
    }

    fn draw_playing(&mut self) -> io::Result<()> {
        self.draw_box(self.outter_box_pos(), self.outter_box_size())?;
        self.draw_box(self.text_box_pos(), self.text_box_size())?;

        self.print_time()?;
        self.print_text_box()?;
        self.go_to_input_beginning()?;
        let word = &self.typed_word;
        write!(stdout(), "{word}")?;
        stdout().flush()?;

        return Ok(());
    }

    fn draw_punct(&mut self) -> io::Result<()> {
        queue!(stdout(), Clear(ClearType::All))?;
        let size = crossterm::terminal::size()?;
        self.draw_box(self.outter_box_pos(), size)?;
        if let None = self.punct {
            self.punct = Some(self.textbox
                .get_punct());
        }

        // This if is always true
        if let Some(p) = &mut self.punct {
            let mut size = size;
            size.0 -= min(size.0, 2);
            size.1 -= min(size.1, 2);
            p.set_pos((1, 1));
            p.set_size(size);
        }

        let punct = self.punct.as_ref().unwrap();
        write!(stdout(), "{punct}")?;
        queue!(stdout(), cursor::Hide)?;
        stdout().flush()?;

        return Ok(());
    }


    fn draw_too_narrow(&mut self) -> io::Result<()> {
        self.go_to_top_left()?;
        queue!(stdout(),
            Clear(ClearType::All),
            Print("\rTOO NARROW. RESIZE.")
        )?;
        stdout().flush()?;
        return Ok(());
    }

    fn draw_box(&mut self, position : (TermUnit, TermUnit), 
        box_size : (TermUnit, TermUnit)) -> io::Result<()> {

        let x = position.0;
        let y = position.1;
        let width = box_size.0;
        let lenght = box_size.1;


        queue!(stdout(), MoveTo(x, y))?;

        let line_width = &"━".repeat(width as usize - 2);
        let top_border = format!("┏{line_width}┓");
        write!(stdout(), "{top_border}")?;

        queue!(stdout(), MoveTo(x, y + 1))?;

        let blank_width = " ".repeat(width as usize - 2);
        for _row in 2 .. lenght {
            queue!(stdout(), MoveToColumn(x))?;
            write!(stdout(), "┃{blank_width}┃")?;
            queue!(stdout(), MoveDown(1))?;
        }

        let bottom_border = format!("┗{line_width}┛");

        queue!(stdout(), MoveToColumn(x))?;
        write!(stdout(), "{bottom_border}")?;
        return Ok(());
    }

    // REFACTOR
    fn type_key_event(&mut self, key : KeyEvent) -> io::Result<bool> {
        if self.is_ended() { 
            if let KeyCode::Esc = key.code {
                return Ok(false);
            } else {
                return Ok(true);
            }
        }
        if self.is_too_narrow() { return Ok(true) }
        let shift = KeyModifiers::from_name("SHIFT").unwrap();
        let none = KeyModifiers::empty();
        if key.modifiers != shift &&
        key.modifiers != none { return Ok(true) }
        match key.code {
            KeyCode::Char(c) => {
                self.go_to_text()?;
                self.textbox.type_char(c)?;
                if c.is_whitespace() {
                    self.typed_word.clear();
                    self.input_offset = self.box_info.left_padding;
                } else {
                    self.typed_word.push(c);
                    self.input_offset = self.box_info.left_padding + std::cmp::min(
                        self.typed_word.chars().count() as u16, self.width - 1);
                }
                self.print_input()?;
            },

            KeyCode::Esc => return Ok(false),

            KeyCode::Backspace => {
                self.go_to_text()?;
                self.textbox.backspace()?;
                self.typed_word.pop().map(|_| self.input_offset -= 1);
                self.print_input()?;
            },

            KeyCode::Right => {
                self.stop();
                self.width += 4;
                self.draw()?;
            },

            KeyCode::Left => {
                self.stop();
                self.width -= std::cmp::min(self.width, 4);
                self.draw()?;
            },

            KeyCode::Down => {
                self.stop();
                self.lines_to_show += 1;
                self.draw()?;
            },

            KeyCode::Up => {
                self.stop();
                self.lines_to_show -= std::cmp::min(self.lines_to_show, 1);
                self.draw()?;
            },

            _ => (),
        }
        return Ok(true);
    }

    fn print_text_box(&mut self) -> io::Result<()> {
        self.go_to_text()?;
        let textbox = &self.textbox;
        write!(stdout(), "{textbox}")?;
        self.go_to_input()?;
        return Ok(());
    }

    fn print_time(&mut self) -> io::Result<()> {
        if self.is_too_narrow() { return Ok(()) }
        let secs = self.textbox.get_remaining_time().as_secs() % 60;
        let mins = self.textbox.get_remaining_time().as_secs() / 60;
        self.go_to_time()?;
        write!(stdout(), "{mins:0>2}:{secs:0>2}")?; 
        self.go_to_input()?;
        stdout().flush()?;
        return Ok(());
    }

    fn print_input(&mut self) -> io::Result<()> {
        self.go_to_input_beginning()?;
        self.print_blank()?;
        self.go_to_input_beginning()?;
        let max = std::cmp::min(self.typed_word.len(), self.width as usize);
        if max > 0 {
            let input_but_last : String= self.typed_word.chars().take(max - 1).collect();
            write!(stdout(), "{input_but_last}")?;
        }
        if let Some(c) = self.typed_word.chars().last() {
            write!(stdout(), "{c}")?;
        }
        self.go_to_input()?;
        return Ok(());
    }

    fn print_blank(&mut self) -> io::Result<()> {
        let empty_space = " ".repeat(self.box_info.width as usize);
        write!(stdout(), "{empty_space}")?;
        return Ok(());
    }

    fn go_to_input(&mut self) -> io::Result<()> {
        let lines_to_show = self.lines_to_show();
        queue!(stdout(),
            MoveTo(self.input_offset
                , self.box_info.top_padding + lines_to_show + 1))?;
        return Ok(());
    }

    fn go_to_input_beginning(&mut self) -> io::Result<()> {
        let lines_to_show = self.lines_to_show();
        queue!(stdout(),
            MoveTo( self.box_info.left_padding, 
                    self.box_info.top_padding + lines_to_show + 1))?;
        return Ok(());
    }

    fn go_to_text(&mut self) -> io::Result<()> {
        queue!(stdout(),
            MoveTo(self.box_info.left_padding,
                self.box_info.top_padding))?;
        return Ok(());
    }

    fn go_to_time(&mut self) -> io::Result<()> {
        return queue!(stdout(), 
            MoveTo(self.box_info.left_padding, self.box_info.top_padding - 2));
    }

    fn go_to_top_left(&mut self) -> io::Result<()> {
        queue!(stdout(),
            MoveTo(0, 0))?;
        return Ok(());
    }

    fn outter_box_pos(&self) -> (TermUnit, TermUnit) {
        return (0, 0);
    }

    fn outter_box_size(&self) -> (TermUnit, TermUnit) {
        return self.box_info.size;
    }

    fn text_box_pos(&self) -> (TermUnit, TermUnit) {
        return (self.box_info.left_padding - 1, self.box_info.top_padding - 1);
    }

    fn text_box_size(&self) -> (TermUnit, TermUnit) {
        return (self.box_info.width + 2, self.lines_to_show() + 2);
    }

    fn lines_to_show(&self) -> TermUnit {
        return std::cmp::min::<TermUnit>(
            self.box_info.size.1 / 2 - 2,
            self.lines_to_show);
    }

    fn event_read(&mut self) -> io::Result<bool> {
        if let Ok(event) = read() {
            match event {
                Event::Key(k) => {
                    if !self.is_too_narrow() {
                        return self.type_key_event(k);
                    } 
                },
                Event::Resize(_, _) => {
                    if !self.is_ended() {
                        self.stop();
                    }
                    self.draw()?;
                }
                Event::FocusGained => {
                    if !self.is_ended() {
                        self.run();
                    }
                }
                Event::FocusLost => {
                    if !self.is_ended() {
                        self.stop();
                    }
                }

                _ => (),
            }
        }
        return Ok(true);
    }
}

impl Drop for Mecano {
    fn drop (&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
    }
}

#[cfg(test)]
mod test {
    use std::{io::{stdout, Write}, thread, time::Duration};

    use crossterm::event::{poll, KeyCode, KeyEvent, KeyModifiers};

    use crate::{config::Config, engine::Mecano, path_to_file};


    #[test]
    // REFACTOR
    fn keys_all_right() {
        let config = Config::default_test();
        let mut state = Mecano::new(config).unwrap();

        let _ = state.draw();
        let _ = stdout().flush();
        let frame_duration = Duration::from_millis(100);

        let find_path_to_file = path_to_file("100_english").unwrap();
        let contents = std::fs::read_to_string(find_path_to_file)
            .unwrap();
        let mut text : String = String::new();
        for word in contents.split_whitespace() {
            text.push_str(word);
            text.push(' ');
        }
        let mut iter = text.chars();
        let mut spaces = 0;

        state.run();
        while !state.is_ended() {

            if let Ok(true) = poll(Duration::ZERO) {
                break;
            }

            let c = iter.next().unwrap_or_else(|| {
                iter = text.chars();
                return iter.next().unwrap();
            });

            if c == ' ' {
                spaces += 1;
            }

            let keyevent = 
            KeyEvent::new(KeyCode::Char(c)
                , KeyModifiers::empty());

            let _ = state.type_key_event(keyevent);

            let _ = state.update_time(frame_duration);
        }

        let punct = state.textbox.get_punct();

        let (c_right, c_wrong, c_extra, c_missed, raw, wpm, acc) =
        punct.get_raw_info();

        assert_eq!(c_wrong, 0);
        assert_eq!(c_extra, 0);
        assert_eq!(c_missed, 0);
        assert_eq!(c_right, (1.0 / frame_duration.as_secs_f64() * 60.0) as u64 - spaces);
        assert_eq!(raw, wpm);
        assert_eq!(acc, 1.0);
    }



    #[test]
    fn too_narrow() {

        let config = Config::default_test();
        let mut state = Mecano::new(config).unwrap();
        let _ = state.draw();


        let mut make_widder = vec![ 
            KeyEvent::new(KeyCode::Right, KeyModifiers::empty()); 
            50];

        let mut make_narrower = vec![ 
            KeyEvent::new(KeyCode::Right, KeyModifiers::empty()); 
            50];

        let frame_duration = Duration::from_millis(100);

        while !state.is_ended() {

            thread::sleep(Duration::from_millis(1));

            if let Ok(true) = poll(Duration::ZERO) {
                break;
            }

            let keyevent;

            if let Some(k) = make_widder.pop() {
                keyevent = k;
            } else if let Some (k) = make_narrower.pop() {
                keyevent = k;
            } else {
                break;
            }

            let _ = state.type_key_event(keyevent);

            let _ = state.update_time(frame_duration);
        }
    }

    #[test]
    #[ignore]
    fn infinite() {
        let config = Config::max_time();
        let mut state = Mecano::new(config).unwrap();
        let _ = state.draw();

        let find_path_to_file = path_to_file("100_english").unwrap();
        let contents = std::fs::read_to_string(find_path_to_file)
            .unwrap();
        let mut text : String = String::new();
        for word in contents.split_whitespace() {
            text.push_str(word);
            text.push(' ');
        }

        let mut iter = text.chars();

        let frame_duration = Duration::from_millis(100);

        while !state.is_ended() {

            if let Ok(true) = poll(Duration::ZERO) {
                break;
            }

            let c = iter.next().unwrap_or_else(|| {
                iter = text.chars();
                return iter.next().unwrap();
            });

            let keyevent = 
                KeyEvent::new(KeyCode::Char(c), KeyModifiers::empty());

            let _ = state.type_key_event(keyevent);

            let _ = state.update_time(frame_duration);
        }

    }
}
