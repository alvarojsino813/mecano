use crate::modes::Mode;
use std::{
    cmp::min, collections::VecDeque, io::{self, stdout, Write}, time::Duration};

use crossterm::{
    cursor::{self, MoveDown, MoveLeft, MoveTo, MoveToColumn, MoveUp}, 
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    queue,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};


use crate::config::Config;

use super::{
    drawing::{draw_box, draw_too_narrow, print_empty_width, BoxInfo}, 
    line::{MecanoLine, WordState}};

#[derive(Clone, Copy)]
enum Screen {
    DictMode,
    Punctuation,
    TooNarrow,
}

pub struct State {
    input_offset : u16,
    typed_word : String,
    lines : VecDeque<MecanoLine>,
    n_total_words : usize,
    n_correct_words : usize,
    words_source : Mode,
    box_info : BoxInfo,
    config : Config,
    end : bool,
    resized : bool,
    actual_time : Duration,
    screen : Screen, 
}

impl State {
    pub fn start(config : Config) -> io::Result<State> {

        // TODO: permitir contents vacios y con varias lineas
        // TODO: No utilizar unwrap
        let words_source = 
        Mode::new(&config.mode, &config.file, config.width);
        let mut words_source = words_source.unwrap();

        let _ = crossterm::terminal::enable_raw_mode();
        let _ = execute!(stdout(), EnterAlternateScreen);

        let actual_time = config.get_max_time(); 

        let mut lines : VecDeque<MecanoLine> = VecDeque::new();
        for _ in 0..config.lines_to_show {
            let mecano_line = MecanoLine::new(
                words_source.yield_words(),
                config.config_line
            );
            lines.push_back(mecano_line);
        }

        lines[0].select();

        let mut state : State = State {
            words_source,
            typed_word : String::new(),
            input_offset : 0,
            lines,
            n_total_words : 0,
            n_correct_words : 0,
            box_info : BoxInfo::default(),
            end : false,
            resized : false,
            config,
            actual_time,
            screen : Screen::DictMode,
        };

        state.draw()?;

        stdout().flush()?;

        return Ok(state);
    }

    pub fn draw(&mut self) -> io::Result<()> {

        queue!(stdout(), Clear(ClearType::All))?;
        let real_size = crossterm::terminal::size().unwrap();
        self.resized = real_size != self.box_info.size;

        if let Ok(box_info) = BoxInfo::centered(self.config.width, real_size) {
            self.input_offset -= self.box_info.left_padding;
            self.box_info = box_info;
            self.input_offset += self.box_info.left_padding;
            self.screen = if self.end { Screen::Punctuation } else { Screen::DictMode }
        } else {
            self.screen = Screen::TooNarrow;
        }

        match self.screen {
            Screen::DictMode => {
                self.draw_dict_mode()?;
            }
            Screen::Punctuation => {
                self.draw_punct()?;
            }
            Screen::TooNarrow => {
                draw_too_narrow()?;
            }
        }

        return Ok(());
    }

    fn draw_dict_mode(&mut self) -> io::Result<()> {
        draw_box((0, 0), self.box_info.size)?;
        draw_box(
            (self.box_info.left_padding - 1, self.box_info.top_padding - 1),
            (self.box_info.width + 2, self.lines_to_show() as u16 + 2))?;
        self.print_time()?;
        self.print_lines()?;
        queue!(stdout(), MoveToColumn(self.box_info.left_padding))?;
        write!(stdout(), "{}", self.typed_word)?;
        stdout().flush()?;

        return Ok(());
    }

    fn draw_punct(&mut self) -> io::Result<()> {
        draw_box((0, 0), self.box_info.size,)?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2 - 8, self.box_info.size.1 / 2))?;
        write!(stdout(), "Total WPM:   {}",
            self.n_total_words as u64 * 60 / self.config.get_max_time().as_secs())?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2 - 8, self.box_info.size.1 / 2 + 1))?;
        write!(stdout(), "Correct WPM: {}", 
            self.n_correct_words as u64 * 60 / self.config.get_max_time().as_secs())?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2 - 8, self.box_info.size.1 / 2 + 3))?;
        write!(stdout(), "<ESC> to exit")?;
        queue!(stdout(), cursor::Hide)?;
        stdout().flush()?;
        return Ok(());
    }

    pub fn type_key_event(&mut self, key : KeyEvent) -> io::Result<()> {
        if self.end { return Ok(()); }
        if let Screen::TooNarrow = self.screen { return Ok(()); }
        let shift = KeyModifiers::from_name("SHIFT").unwrap();
        let none = KeyModifiers::empty();
        if key.modifiers != shift &&
        key.modifiers != none { return Ok(()); }
        match key.code {
            KeyCode::Char(c) => {
                self.type_char(c)?;
            },

            KeyCode::Backspace => {
                self.backspace()?;
            },

            _ => (),
        }
        return Ok(());
    }

    pub fn end(&mut self) {
        self.end = true;
        self.screen = Screen::Punctuation;
        let _ = self.draw();
    }

    pub fn is_ended(&self) -> bool {
        return self.end;
    }

    pub fn is_resized(&self) -> bool {
        return self.resized;
    }

    fn type_char(&mut self, c : char) -> io::Result<()> {
        if c.is_whitespace() {
            let next_word = self.lines[0].next_word(&self.typed_word);
            if let Ok(word_state) = next_word {
                self.n_total_words += 1;
                if let WordState::Correct = word_state {
                    self.n_correct_words += 1;
                }
            } else {
                self.n_total_words += 1;
                if let Err(WordState::Correct) = next_word {
                    self.n_correct_words += 1;
                }
                self.next_line()?;
            }
            self.typed_word.clear();
            self.input_offset = self.box_info.left_padding;
            print_empty_width(self.box_info.left_padding, self.box_info.width)?;
            self.print_selected_line()?;
        } else if !c.is_control() {
            self.typed_word.push(c);
            write!(stdout(), "{}", c)?;
            if self.input_offset + 1 < self.box_info.left_padding + self.box_info.width {
                self.input_offset += 1;
            }
            self.lines[0].typed(&self.typed_word);
            self.print_selected_line()?;
        } 
        stdout().flush()?;
        return Ok(());
    }

    fn print_selected_line(&mut self) -> io::Result<()> {
        queue!(stdout(), 
            MoveUp(self.lines_to_show() as u16 + 1),
            MoveToColumn(self.box_info.left_padding))?;

        write!(stdout(), "{}", self.lines[0])?;

        self.go_to_input()?;

        return Ok(());
    }

    fn backspace(&mut self) -> io::Result<()> {
        if self.input_offset > self.box_info.left_padding {
            self.input_offset -= 1;
            queue!(stdout(), MoveLeft(1))?;
            write!(stdout(), " ")?;
            queue!(stdout(), MoveLeft(1))?;
            self.typed_word.pop();
            self.lines[0].typed(&self.typed_word);
            self.print_selected_line()?;
            stdout().flush()?;
        }
        return Ok(());
    }

    fn next_line(&mut self) -> io::Result<()> {
        self.lines.pop_front(); 
        self.lines.push_back(MecanoLine::new(
            self.words_source.yield_words(),
            self.config.config_line
        ));
        self.input_offset = self.box_info.left_padding;
        self.lines[0].select(); 
        self.print_lines()?;
        return Ok(());
    }

    fn print_lines(&mut self) -> io::Result<()> {

        queue!(stdout(),
            MoveTo(self.box_info.left_padding, self.box_info.top_padding))?;
        for line in self.lines.iter().take(self.lines_to_show() as usize) {
            print_empty_width(self.box_info.left_padding, self.box_info.width)?;
            queue!(stdout(), 
                MoveToColumn(self.box_info.left_padding))?;
            write!(stdout(), "{}{}{}",
                line,
                MoveDown(1), 
                MoveToColumn(self.box_info.left_padding))?;
        }
        self.go_to_input()?;
        return Ok(());
    }

    fn print_time(&mut self) -> io::Result<()> {
        if let Screen::TooNarrow = self.screen { return Ok(()); }
        queue!(stdout(), 
            MoveTo(self.box_info.left_padding, self.box_info.top_padding - 2))?;
        let secs = self.actual_time.as_secs() % 60;
        let mins = self.actual_time.as_secs() / 60;
        write!(stdout(), "{mins:0>2}:{secs:0>2}")?; 
        self.go_to_input()?;
        stdout().flush()?;

        return Ok(());
    }

    pub fn update_time(&mut self, elapsed : Duration) -> io::Result<()> {
        self.actual_time -= min(elapsed, self.actual_time);
        self.print_time()?;
        if self.actual_time == Duration::from_secs(0) {
            self.end()
        }
        return Ok(());
    }

    fn go_to_input(&mut self) -> io::Result<()> {
        queue!(stdout(),
            MoveTo(self.input_offset,
                self.box_info.top_padding + self.lines_to_show() as u16 + 1))?;
        return Ok(());
    }

    fn lines_to_show(&self) -> u16 {
        return std::cmp::min::<u16>(
            self.box_info.size.1 / 2 - 2,
            self.config.lines_to_show as u16);
    }
}

impl Drop for State {
    fn drop (&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(stdout(), Clear(ClearType::All),
            LeaveAlternateScreen, cursor::Show);
    }
}

#[cfg(test)]

mod test {
    use std::thread;
    use std::time::Duration;

    use super::State;
    use crate::find_path_to_file;
    use crate::mecano::Config;


    #[test]
    fn typing_normally() {
        let mut config = Config::_default();
        config.file = find_path_to_file("1_test").expect("1_test file not found");
        config.mode = "dictionary".to_string();
        let mut state = State::start(config).unwrap();
        let keys = [
            'H', 'o', 'l', 'a', ' ', 
            'q', 'u', 'e', ' ',
            't', 'a', 'l', ' ',
            'H', 'o', 'l', 'a', ' ', 
            'H', 'o', 'l', 'a', ' ', 
        ];
        for i in keys {
            thread::sleep(Duration::from_millis(5));
            let _ = state.type_char(i);
        }

        thread::sleep(Duration::from_millis(500));
        assert_eq!(state.n_correct_words, 3);
        assert_eq!(state.n_total_words, 5);
    }
}
