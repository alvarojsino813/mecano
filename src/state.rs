use std::{fmt::Display, io::{self, Stdout, Write}};
use crossterm::{
    cursor::{Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToNextLine, MoveUp}, 
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    queue,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};
use dictionary::Dictionary;
use crate::config::Config;
use crate::dictionary;

struct BoxInfo {
    left_padding : u16,
    top_padding : u16,
    width : u16,
    size : (u16, u16),
}

impl BoxInfo {
    pub fn centered() -> BoxInfo {
        let size = crossterm::terminal::size().unwrap();
        let width = 70;
        let left_padding = (size.0 - width) / 2;
        let top_padding = (size.1 - 4) / 2;

        BoxInfo {
            left_padding,
            top_padding,
            width,
            size,
        }
    }
}

#[derive(Clone, Copy)]
enum WordState {
    Correct,
    Wrong,
    Selected,
    TypingWrong,
    Unreached,
}

pub struct State {
    stdout : Stdout,
    actual_offset : u16,
    input_offset : u16,
    word_offset: u16,
    typed_word : String,
    actual_line : Vec<String>,
    actual_word_state : Vec<WordState>,
    second_line : Vec<String>,
    nth_word : usize,
    n_total_words : usize,
    n_correct_words : usize,
    n_lines : usize,
    dict : Dictionary,
    box_info : BoxInfo,
    config : Config,
    end : bool
}

impl State {
    pub fn start(path_to_dict : &str) -> io::Result<State> {
        // TODO: permitir contents vacios y con varias lineas
        let dict = Dictionary::new(path_to_dict);

        let term_info = BoxInfo::centered();
        let actual_offset = term_info.left_padding;
        let input_offset = term_info.left_padding;
        let word_offset = term_info.left_padding;
        let mut stdout = io::stdout();

        let _ = crossterm::terminal::enable_raw_mode();
        let _ = execute!(stdout, EnterAlternateScreen);

        let mut state : State = State {
            stdout,
            dict,
            typed_word : String::new(),
            actual_line : Vec::new(),
            second_line : Vec::new(),
            actual_word_state : Vec::new(),
            nth_word : 0,
            actual_offset,
            input_offset,
            word_offset,
            n_total_words : 0,
            n_correct_words : 0,
            n_lines : 0,
            box_info: term_info,
            end : false,
            config : Config::default(),
        };

        state.next_line()?;
        state.next_line()?;

        state.draw_dict_mode()?;

        state.highlight_actual_word(state.get_bg_selected())?;
        state.stdout.flush()?;


        return Ok(state);
    }

    pub fn draw_dict_mode(&mut self) -> io::Result<()> {

        self.input_offset -= self.box_info.left_padding;
        self.word_offset -= self.box_info.left_padding;
        self.actual_offset -= self.box_info.left_padding;
        self.box_info = BoxInfo::centered();
        self.input_offset += self.box_info.left_padding;
        self.word_offset += self.box_info.left_padding;
        self.actual_offset += self.box_info.left_padding;

        queue!(self.stdout, Clear(ClearType::All))?;
        self.draw_box()?;
        self.print_lines()?;
        self.stdout.flush()?;
        return Ok(());
    }
    pub fn draw_punct(&mut self) -> io::Result<()> {

        self.box_info = BoxInfo::centered();

        queue!(self.stdout, Clear(ClearType::All))?;
        self.draw_box()?;
        queue!(self.stdout, MoveTo(self.box_info.size.0 / 2, self.box_info.size.1 / 2))?;
        write!(self.stdout, "WPM: {}", self.n_total_words)?;
        self.stdout.flush()?;
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
    }

    pub fn get_size(&self) -> (u16, u16) {
        return self.box_info.size;
    }

    fn type_char(&mut self, c : char) -> io::Result<()> {
        if c.is_whitespace() {
            self.next_word()?;

        } else if !c.is_control() {
            self.typed_word.push(c);
            self.actual_offset += 1;
            self.input_offset += 1;

            write!(self.stdout, "{}", c)?;
            if self.is_typed_correct() {
                self.highlight_actual_word(self.get_bg_selected())?;
                self.actual_word_state[self.nth_word] = WordState::Selected;
            } else {
                self.highlight_actual_word(self.get_bg_wrong())?;
                self.actual_word_state[self.nth_word] = WordState::TypingWrong;
            }
        } else {

            self.typed_word.push(c);
        }

        self.stdout.flush()?;
        return Ok(());
    }

    fn backspace(&mut self) -> io::Result<()> {
        queue!(self.stdout, MoveLeft(1))?;
        write!(self.stdout, " ")?;
        queue!(self.stdout, MoveLeft(1))?;
        if self.actual_offset > self.box_info.left_padding &&
        self.input_offset > self.box_info.left_padding {
            self.actual_offset -= 1;
            self.input_offset -= 1;
            self.typed_word.pop();
        }
        if self.is_typed_correct() {
            self.highlight_actual_word(self.get_bg_selected())?;
        } else {
            self.highlight_actual_word(self.get_bg_wrong())?;
        }
        self.stdout.flush()?;
        return Ok(());
    }

    fn is_typed_correct(&self) -> bool {
        return if 
        self.input_offset as usize - self.box_info.left_padding as usize
        <= 
        self.get_actual_word().len() {
            let mut actual_chars = self.get_actual_word().chars();
            self.typed_word.chars().all(|c| c == actual_chars.next().unwrap_or('\0'))
        } else {
            false
        }
    }

    fn next_word(&mut self) -> io::Result<()> {
        if self.nth_word + 1 >= self.actual_line.len() {
            self.next_line()?;
        } else {
            if self.get_actual_word() == &self.typed_word {
                self.n_correct_words += 1;
                self.actual_word_state[self.nth_word] = WordState::Correct;
                self.color_actual_word(self.get_fg_correct())?;
            } else {
                self.actual_word_state[self.nth_word] = WordState::Wrong;
                self.color_actual_word(self.get_fg_wrong())?;
            }
            self.word_offset += self.get_actual_word().chars().count() as u16 + 1;
            self.nth_word += 1; 
            self.actual_word_state[self.nth_word] = WordState::Selected;
            self.actual_offset += 1;
            self.input_offset = self.box_info.left_padding;
        }

        self.highlight_actual_word(self.get_bg_selected())?;
        self.print_empty_width()?;
        self.go_to_input_offset()?;

        self.typed_word = String::new();
        self.n_total_words += 1;

        return Ok(());
    }

    fn next_line(&mut self) -> io::Result<()> {
        self.actual_line = self.second_line.to_owned();
        self.second_line = self.dict.yield_words(self.box_info.width);

        self.actual_word_state = vec![WordState::Selected];
        self.actual_word_state
            .append(&mut vec![WordState::Unreached; self.actual_line.len()]);

        self.word_offset = self.box_info.left_padding;
        self.input_offset = self.box_info.left_padding;
        self.n_lines += 1;
        self.nth_word = 0;

        self.print_lines()?;
        return Ok(());
    }


    fn print_lines(&mut self) -> io::Result<()> {
        queue!(self.stdout,
            MoveTo(self.box_info.left_padding, self.box_info.top_padding))?;
        self.print_empty_width()?;
        self.go_to_left_pad()?;
        for (word, state) in self.actual_line
            .iter().zip(self.actual_word_state.iter()) {

            match state {
                WordState::Correct => {
                    queue!(self.stdout, 
                        SetForegroundColor(self.config.get_fg_correct()))?;
                    write!(self.stdout, "{}", word)?;
                    queue!(self.stdout, 
                        SetForegroundColor(Color::Reset))?;
                    write!(self.stdout, " ")?;
                },
                WordState::Wrong => {
                    queue!(self.stdout, 
                        SetForegroundColor(self.config.get_fg_wrong()))?;
                    write!(self.stdout, "{}", word)?;
                    queue!(self.stdout, 
                        SetForegroundColor(Color::Reset))?;
                    write!(self.stdout, " ")?;
                },
                WordState::Selected => {
                    queue!(self.stdout, 
                        SetBackgroundColor(self.config.get_bg_selected()))?;
                    write!(self.stdout, "{}", word)?;
                    queue!(self.stdout, 
                        SetBackgroundColor(Color::Reset))?;
                    write!(self.stdout, " ")?;
                },
                WordState::TypingWrong => {
                    queue!(self.stdout, 
                        SetBackgroundColor(self.config.get_bg_selected()))?;
                    write!(self.stdout, "{}", word)?;
                    queue!(self.stdout, 
                        SetBackgroundColor(Color::Reset))?;
                    write!(self.stdout, " ")?;
                },
                WordState::Unreached => {
                    write!(self.stdout, "{} ", word)?;
                }
            }
        }

        queue!(self.stdout, MoveDown(1))?;
        self.print_empty_width()?;
        self.go_to_left_pad()?;
        write!(self.stdout, "{}\r", vec_to_str(&self.second_line))?;

        queue!(self.stdout, MoveDown(2))?;
        self.go_to_input_offset()?;
        return Ok(());
    }

    fn draw_box(&mut self) -> io::Result<()> {
        let mut string = String::from("┏");
        string.push_str("━".repeat(self.box_info.size.0 as usize - 2).as_str());
        string.push('┓');
        queue!(self.stdout, MoveTo(0, 0))?;
        write!(self.stdout, "{}", string)?;

        for _row in 2 .. self.box_info.size.0 {
            write!(self.stdout, "┃")?;
            queue!(self.stdout, MoveToColumn(self.box_info.size.0))?;
            write!(self.stdout, "┃")?;
            queue!(self.stdout, MoveToNextLine(1))?;
        }

        let mut string = String::from("┗");
        string.push_str("━".repeat(self.box_info.size.0 as usize - 2).as_str());
        string.push('┛');
        write!(self.stdout, "{}", string)?;
        return Ok(());
    }

    fn highlight_actual_word(&mut self, color : Color) -> io::Result<()> {

        queue!(self.stdout,
            MoveToColumn(0),
            MoveUp(3),
            SetBackgroundColor(color),
            MoveToColumn(0)
        )?;

        self.go_to_left_pad()?;
        let pad = self.word_offset as i32 - self.box_info.left_padding as i32;
        if pad > 0 {
            write!(self.stdout, "{}", MoveRight(pad as u16))?;
        }

        write!(self.stdout, "{}", self.actual_line[self.nth_word])?;
        queue!(self.stdout, 
            MoveDown(3),
            SetBackgroundColor(Color::Reset),
            MoveToColumn(0)
        )?;

        self.go_to_input_offset()?;

        return Ok(());
    }

    fn color_actual_word(&mut self, color : Color) -> io::Result<()> {
        queue!(self.stdout,
            MoveToColumn(0),
            MoveUp(3),
            SetForegroundColor(color),
            MoveToColumn(0)
        )?;

        self.go_to_left_pad()?;
        let pad = self.word_offset as i32 - self.box_info.left_padding as i32;
        if pad > 0 {
            write!(self.stdout, "{}", MoveRight(pad as u16))?;
        }

        write!(self.stdout, "{}", self.actual_line[self.nth_word])?;
        queue!(self.stdout, 
            MoveDown(3),
            SetForegroundColor(Color::Reset),
            MoveToColumn(0)
        )?;

        self.go_to_input_offset()?;

        return Ok(());
    }

    fn go_to_left_pad(&mut self) -> io::Result<()> {
        write!(self.stdout, "\r")?;
        if self.box_info.left_padding > 0 {
            write!(self.stdout, "{}", MoveRight(self.box_info.left_padding))?;
        }
        return Ok(());
    }

    fn go_to_input_offset(&mut self) -> io::Result<()> {
        write!(self.stdout, "\r")?;
        if self.input_offset > 0 {
            write!(self.stdout, "{}", MoveRight(self.input_offset))?;
        }
        return Ok(());
    }

    fn print_empty_width(&mut self) -> io::Result<()> {
        queue!(self.stdout, MoveToColumn(2))?; 
        let empty = " ".repeat(self.box_info.size.0 as usize - 3);
        write!(self.stdout, "{}", empty)?;
        self.stdout.flush()?;
        Ok(())
    }

    fn get_actual_word(&self) -> &str {
        return &self.actual_line[self.nth_word];
    }

    fn get_bg_selected(&self) -> Color {
        return self.config.get_bg_selected()
    }

    fn get_bg_wrong(&self) -> Color {
        return self.config.get_bg_wrong();
    }

    fn get_fg_wrong(&self) -> Color {
        return self.config.get_fg_wrong();
    }

    fn get_fg_correct(&self) -> Color {
        return self.config.get_fg_correct();
    }

    fn print_debug(&mut self) -> io::Result<()> {

        let empty = "                                                                                                                                   ";

        let mut debug_info : String = String::new();

        debug_info.push_str(format!("\r{}\ractual_word :       {}\n",
            empty, self.get_actual_word()).as_str());

        debug_info.push_str(format!("\r{}\rtyped_word :        {}\n",
            empty, self.typed_word).as_str());

        debug_info.push_str(format!("\r{}\rn_correct_words :   {}\n",
            empty, self.n_correct_words).as_str());

        debug_info.push_str(format!("\r{}\rn_words :           {}\n",
            empty, self.n_total_words).as_str());

        debug_info.push_str(format!("\r{}\rinput_offset :      {}\n",
            empty, self.input_offset).as_str());

        debug_info.push_str(format!("\r{}\ractual_offset :     {}\n",
            empty, self.actual_offset).as_str());

        debug_info.push_str(format!("\r{}\rword_offset :       {}\n",
            empty, self.word_offset).as_str());

        debug_info.push_str(format!("\r{}\ractual_line :       {}\n",
            empty, vec_to_str(&self.actual_line)).as_str());

        debug_info.push_str(format!("\r{}\rsecond_line :       {}\n",
            empty, vec_to_str(&self.second_line)).as_str());

        debug_info.push_str(format!("\r{}\rnth_word :          {}\n",
            empty, self.nth_word).as_str());

        debug_info.push_str(format!("\r{}\rwords.len() :       {}\n",
            empty, self.actual_line.len()).as_str());

        if self.input_offset == 0 {
            write!(self.stdout, "{}{}{}\r",
                MoveDown(2),
                debug_info,
                MoveUp(debug_info.lines().count() as u16 + 2))?;
        } else {
            write!(self.stdout, "{}{}{}\r{}",
                MoveDown(2),
                debug_info,
                MoveUp(debug_info.lines().count() as u16 + 2),
                MoveRight(self.input_offset))?;
        }
        self.stdout.flush()?;
        return Ok(());
    }
}

impl Drop for State {
    fn drop (&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(self.stdout, Clear(ClearType::All));
        let _ = execute!(self.stdout, LeaveAlternateScreen);
    }
}

fn vec_to_str(vec : &Vec<String>) -> String {
    let mut string = String::new();

    for word in vec {
        string.push_str(word); 
        string.push(' '); 
    }

    return string;
}
