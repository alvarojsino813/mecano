use std::io::{self, Stdout, Write};
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::raw::IntoRawMode;
use dictionary::Dictionary;

use crate::config::Config;
use crate::dictionary;

struct TermInfo {
    left_padding : u16,
    top_padding : u16,
    width : u16,
    size : (u16, u16),
}

impl TermInfo {
    pub fn centered() -> TermInfo {
        let size = termion::terminal_size().unwrap();
        let width = 40;
        let left_padding = (size.0 - width) / 2;
        let top_padding = (size.1 - 4) / 2;

        TermInfo {
            left_padding,
            top_padding,
            width,
            size,
        }
    }
}

pub struct State {
    stdout : termion::raw::RawTerminal<Stdout>,
    actual_offset : u16,
    input_offset : u16,
    word_offset: u16,
    typed_word : String,
    actual_line : Vec<String>,
    second_line : Vec<String>,
    nth_word : usize,
    n_total_words : usize,
    n_correct_words : usize,
    n_lines : usize,
    dict : Dictionary,
    term_info : TermInfo,
    config : Config,
}

impl State {
    pub fn start(path_to_dict : &str) -> io::Result<State> {
        // TODO: permitir contents vacios y con varias lineas
        let dict = Dictionary::new(path_to_dict);

        let term_info = TermInfo::centered();
        let actual_offset = term_info.left_padding;
        let input_offset = term_info.left_padding;
        let word_offset = term_info.left_padding;

        print!("{}", termion::clear::All);
        let stdout = io::stdout().into_raw_mode()?;

        let mut state : State = State {
            stdout,
            dict,
            typed_word : String::new(),
            actual_line : Vec::new(),
            second_line : Vec::new(),
            nth_word : 0,
            actual_offset,
            input_offset,
            word_offset,
            n_total_words : 0,
            n_correct_words : 0,
            n_lines : 0,
            term_info,
            config : Config::default(),
        };

        state.next_line()?;
        state.next_line()?;

        state.draw()?;

        state.highlight_actual_word(state.get_bg_selected())?;
        state.stdout.lock().flush()?;


        return Ok(state);
    }

    pub fn draw(&mut self) -> io::Result<()> {
        write!(self.stdout.lock(), "{}", termion::clear::All)?;

        self.input_offset -= self.term_info.left_padding;
        self.word_offset -= self.term_info.left_padding;
        self.actual_offset -= self.term_info.left_padding;
        self.term_info = TermInfo::centered();
        self.input_offset += self.term_info.left_padding;
        self.word_offset += self.term_info.left_padding;
        self.actual_offset += self.term_info.left_padding;

        self.stdout.lock().flush()?;

        let string = "-".repeat(self.term_info.size.0 as usize);
        write!(self.stdout.lock(), "{}{}", termion::cursor::Goto(1, 1), string)?;
        self.print_lines()?;
        return Ok(());
    }

    pub fn type_key(&mut self, key : Key) -> io::Result<()> {
        match key {
            Key::Char(c) => {
                self.type_char(c)?;
            },

            Key::Backspace => {
                self.backspace()?;
            }

            _ => (),
        }
        return Ok(());
    }

    pub fn get_size(&self) -> (u16, u16) {
        return self.term_info.size;
    }

    fn type_char(&mut self, c : char) -> io::Result<()> {
        if c.is_whitespace() {
            self.next_word()?;

        } else if !c.is_control() {
            self.typed_word.push(c);
            self.actual_offset += 1;
            self.input_offset += 1;

            write!(self.stdout.lock(), "{}", c)?;
            if self.is_typed_correct() {
                self.highlight_actual_word(self.get_bg_selected())?;
            } else {
                self.highlight_actual_word(self.get_bg_wrong())?;
            }
        } else {

            self.typed_word.push(c);
        }

        self.stdout.lock().flush()?;
        return Ok(());
    }

    fn backspace(&mut self) -> io::Result<()> {
        write!(self.stdout.lock(), "{} {}", cursor::Left(1), cursor::Left(1))?;
        if self.actual_offset > self.term_info.left_padding &&
        self.input_offset > self.term_info.left_padding {
            self.actual_offset -= 1;
            self.input_offset -= 1;
            self.typed_word.pop();
        }
        if self.is_typed_correct() {
            self.highlight_actual_word(self.get_bg_selected())?;
        } else {
            self.highlight_actual_word(self.get_bg_wrong())?;
        }
        self.stdout.lock().flush()?;
        return Ok(());
    }

    fn is_typed_correct(&self) -> bool {
        return if 
        self.input_offset as usize - self.term_info.left_padding as usize
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
            self.nth_word = 0;
        } else {
            if self.get_actual_word() == &self.typed_word {
                self.n_correct_words += 1;
                self.color_actual_word(self.get_fg_correct())?;
            } else {
                self.color_actual_word(self.get_fg_wrong())?;
            }
            self.word_offset += self.get_actual_word().chars().count() as u16 + 1;
            self.nth_word += 1; 
            self.actual_offset += 1;
            self.input_offset = self.term_info.left_padding;
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
        self.second_line = self.dict.yield_words(self.term_info.width);

        self.word_offset = self.term_info.left_padding;
        self.input_offset = self.term_info.left_padding;
        self.n_lines += 1;

        self.print_lines()?;
        return Ok(());
    }


    fn print_lines(&mut self) -> io::Result<()> {
        write!(self.stdout.lock(), "{}",
            cursor::Goto(self.term_info.left_padding, self.term_info.top_padding))?;
        self.print_empty_width()?;
        self.go_to_left_pad()?;
        write!(self.stdout.lock(), "{}\r", vec_to_str(&self.actual_line))?;

        write!(self.stdout.lock(), "{}", cursor::Down(1))?;
        self.print_empty_width()?;
        self.go_to_left_pad()?;
        write!(self.stdout.lock(), "{}\r", vec_to_str(&self.second_line))?;

        write!(self.stdout.lock(), "{}", cursor::Down(2))?;
        self.go_to_input_offset()?;
        self.stdout.lock().flush()?;
        return Ok(());
    }

    fn highlight_actual_word(&mut self, color : color::Rgb) -> io::Result<()> {

        write!(self.stdout.lock(), "\r{}{}\r",
            cursor::Up(3),
            color::Bg(color),
        )?;

        self.go_to_left_pad()?;
        let pad = self.word_offset as i32 - self.term_info.left_padding as i32;
        if pad > 0 {
            write!(self.stdout.lock(), "{}", cursor::Right(pad as u16))?;
        }

        write!(self.stdout.lock(), "{}{}{}\r",
            self.actual_line[self.nth_word],
            cursor::Down(3),
            color::Bg(color::Reset),
        )?;

        self.go_to_input_offset()?;

        return Ok(());
    }

    fn color_actual_word(&mut self, color : color::Rgb) -> io::Result<()> {
        write!(self.stdout.lock(), "\r{}{}\r",
            cursor::Up(3),
            color::Fg(color),
        )?;

        self.go_to_left_pad()?;
        let pad = self.word_offset as i32 - self.term_info.left_padding as i32;
        if pad > 0 {
            write!(self.stdout.lock(), "{}", cursor::Right(pad as u16))?;
        }

        write!(self.stdout.lock(), "{}{}{}\r",
            self.actual_line[self.nth_word],
            cursor::Down(3),
            color::Fg(color::Reset),
        )?;

        self.go_to_input_offset()?;

        return Ok(());
    }

    fn go_to_left_pad(&mut self) -> io::Result<()> {
        write!(self.stdout.lock(), "\r")?;
        if self.term_info.left_padding > 0 {
            write!(self.stdout.lock(), "{}", cursor::Right(self.term_info.left_padding))?;
        }
        return Ok(());
    }

    fn go_to_input_offset(&mut self) -> io::Result<()> {
        write!(self.stdout.lock(), "\r")?;
        if self.input_offset > 0 {
            write!(self.stdout.lock(), "{}", cursor::Right(self.input_offset))?;
        }
        return Ok(());
    }

    fn print_empty_width(&mut self) -> io::Result<()> {
        write!(self.stdout.lock(), "{}", termion::clear::UntilNewline)?;
        Ok(())
    }

    fn get_actual_word(&self) -> &str {
        return &self.actual_line[self.nth_word];
    }

    fn get_bg_selected(&self) -> color::Rgb {
        return self.config.get_bg_selected()
    }

    fn get_bg_wrong(&self) -> color::Rgb {
        return self.config.get_bg_wrong();
    }

    fn get_fg_wrong(&self) -> color::Rgb {
        return self.config.get_fg_wrong();
    }

    fn get_fg_correct(&self) -> color::Rgb {
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
            write!(self.stdout.lock(), "{}{}{}\r",
                cursor::Down(2),
                debug_info,
                cursor::Up(debug_info.lines().count() as u16 + 2))?;
        } else {
            write!(self.stdout.lock(), "{}{}{}\r{}",
                cursor::Down(2),
                debug_info,
                cursor::Up(debug_info.lines().count() as u16 + 2),
                cursor::Right(self.input_offset))?;
        }
        self.stdout.lock().flush()?;
        return Ok(());
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
