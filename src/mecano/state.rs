use std::{
    io::{self, Stdout, Write, stdout},
    time::Duration,
    collections::VecDeque};

use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveTo, MoveToColumn, MoveUp}, 
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    queue,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{
    config::Config,
    dictionary::Dictionary};

use super::{
    drawing::{BoxInfo, draw_box, draw_width_warning, print_empty_width}, 
    line::MecanoLine};

#[derive(Clone, Copy)]
enum Screen {
    DictMode,
    Punctuation,
}

pub struct State {
    stdout : Stdout,
    input_offset : u16,
    typed_word : String,
    lines : VecDeque<MecanoLine>,
    n_total_words : usize,
    n_correct_words : usize,
    dict : Dictionary,
    box_info : BoxInfo,
    config : Config,
    end : bool,
    actual_time : Duration,
    screen : Screen, 
}

impl State {
    pub fn start(path_to_dict : &str) -> io::Result<State> {
        // TODO: permitir contents vacios y con varias lineas
        let dict = Dictionary::new(path_to_dict);

        let box_info = BoxInfo::centered().unwrap();
        let input_offset = box_info.left_padding;
        let mut stdout = stdout();

        let _ = crossterm::terminal::enable_raw_mode();
        let _ = execute!(stdout, EnterAlternateScreen);

        let config = Config::default();
        let actual_time = config.get_max_time(); 

        let mut lines : VecDeque<MecanoLine> = VecDeque::new();
        for _ in 0..config.lines_to_show {
            let mecano_line = MecanoLine::new(
                dict.yield_words(box_info.width),
                config.config_line
            );
            lines.push_back(mecano_line);
        }

        lines[0].select();

        let mut state : State = State {
            stdout,
            dict,
            typed_word : String::new(),
            input_offset,
            lines,
            n_total_words : 0,
            n_correct_words : 0,
            box_info,
            end : false,
            config,
            actual_time,
            screen : Screen::DictMode,
        };

        state.draw_dict_mode()?;
        state.print_time()?;

        state.stdout.flush()?;


        return Ok(state);
    }

    pub fn draw(&mut self) -> io::Result<()> {

        match self.screen {
            Screen::DictMode => {
                self.draw_dict_mode()?;
            }
            Screen::Punctuation => {
                self.draw_punct()?;
            }
        }

        return Ok(());
    }

    fn draw_dict_mode(&mut self) -> io::Result<()> {

        if let Ok(box_info) = BoxInfo::centered() {
            self.input_offset -= self.box_info.left_padding;
            self.box_info = box_info;
            self.input_offset += self.box_info.left_padding;
            queue!(stdout(), Clear(ClearType::All))?;
            draw_box((0, 0), self.box_info.size)?;
            draw_box(
                (self.box_info.left_padding - 1, self.box_info.top_padding - 1),
                (self.box_info.width + 2, self.config.lines_to_show as u16 + 2))?;
            self.print_lines()?;
            self.print_time()?;
            stdout().flush()?;
        } else {
            draw_width_warning()?;
        }

        return Ok(());
    }

    fn draw_punct(&mut self) -> io::Result<()> {

        // TO DO -> Sustituir unwrap()
        self.box_info = BoxInfo::centered().unwrap();

        queue!(stdout(), Clear(ClearType::All))?;
        draw_box((0, 0), self.box_info.size,)?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2, self.box_info.size.1 / 2))?;
        write!(stdout(), "WPM: {}", self.n_total_words)?;
        queue!(stdout(),
            MoveTo(self.box_info.size.0 / 2, self.box_info.size.1 / 2 + 1))?;
        write!(stdout(), "WPM (correct): {}", self.n_correct_words)?;
        stdout().flush()?;
        return Ok(());
    }

    pub fn type_key_event(&mut self, key : KeyEvent) -> io::Result<()> {
        if self.end { return Ok(()); }
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

    pub fn get_size(&self) -> (u16, u16) {
        return self.box_info.size;
    }

    fn type_char(&mut self, c : char) -> io::Result<()> {
        if c.is_whitespace() {
            if let Err(()) = self.lines[0].next_word(&self.typed_word) {
                self.next_line()?;
            }
            self.typed_word.clear();
            self.input_offset = self.box_info.left_padding;
            print_empty_width(self.box_info.left_padding, self.box_info.width)?;
            self.print_selected_line()?;
        } else if !c.is_control() {
            self.typed_word.push(c);
            write!(stdout(), "{}", c)?;
            self.input_offset += 1;
            self.lines[0].typed(&self.typed_word);
            self.print_selected_line()?;
        } 
        stdout().flush()?;
        return Ok(());
    }

    fn print_selected_line(&mut self) -> io::Result<()> {
        queue!(stdout(), 
            MoveUp(self.config.lines_to_show as u16 + 1),
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
        self.n_correct_words += self.lines[0].n_correct_words();
        self.n_total_words += self.lines[0].n_total_words();
        self.lines.pop_front(); 
        self.lines.push_back(MecanoLine::new(
            self.dict.yield_words(self.box_info.width),
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
        for line in self.lines.iter() {
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

    pub fn print_time(&mut self) -> io::Result<()> {
        queue!(stdout(), 
            MoveTo(self.box_info.left_padding, self.box_info.top_padding - 2))?;
        let secs = self.actual_time.as_secs() % 60;
        let mins = self.actual_time.as_secs() / 60;
        write!(stdout(), "{mins:0>2}:{secs:0>2}")?; 
        self.go_to_input()?;
        stdout().flush()?;

        return Ok(());
    }

    pub fn sub_sec(&mut self) {
        if self.actual_time >= Duration::from_secs(1) {
            self.actual_time -= Duration::from_secs(1);
            let _ = self.print_time();
        } else {
            self.end()
        }
    }

    fn go_to_input(&mut self) -> io::Result<()> {
        queue!(stdout(),
            MoveTo(self.input_offset,
                self.box_info.top_padding + self.config.lines_to_show as u16 + 1))?;
        return Ok(());
    }
}

impl Drop for State {
    fn drop (&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(stdout(), Clear(ClearType::All),
            LeaveAlternateScreen);
    }
}
