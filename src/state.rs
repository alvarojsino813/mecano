use std::cell::{Cell, RefCell};
use std::io::{self, Stdout, StdoutLock, Write};
use std::str::{Lines, SplitAsciiWhitespace};
use termion::color;
use termion::cursor;
use termion::raw::IntoRawMode;
use dictionary::Dictionary;

use crate::dictionary;

pub enum Mode {
    Dictionary(Dictionary),
    File,
}

pub struct HighlightColor {
    bg_selected : color::Rgb,
    bg_wrong : color::Rgb,
    fg_wrong : color::Rgb,
    fg_correct : color::Rgb,
}

pub struct State<'a> {
    handle : StdoutLock<'static>,
    stdout : termion::raw::RawTerminal<Stdout>,
    actual_offset : u16,
    input_offset : u16,
    word_offset: u16,
    typed_word : String,
    contents : &'a str,
    lines : Lines<'a>,
    actual_line : &'a str,
    words : SplitAsciiWhitespace<'a>,
    actual_word : &'a str,
    n_words : usize,
    n_correct_words : usize,
    n_lines : usize,
    mode : Mode,
    colors : HighlightColor,
}

impl<'a> State<'a> {
    pub fn start(contents : &str) -> io::Result<State> {
        // TODO: permitir contents vacios y con varias lineas
        let mut lines = contents.lines();
        let actual_line = lines.next().unwrap();
        let mut words = actual_line.split_ascii_whitespace();
        let actual_word = words.next().unwrap_or("que esta pasando");
        let stdout = io::stdout().into_raw_mode().unwrap();
        let mut handle = stdout.lock();
        let mode = Mode::File;
        let colors = HighlightColor {
                bg_selected : color::Rgb(128, 128, 128),
                bg_wrong : color::Rgb(200, 128, 128),
                fg_correct : color::Rgb(64, 200, 64),
                fg_wrong : color::Rgb(200, 64, 64),
        };

        write!(handle, "{}\n\n\n\n\n\n{}\r", actual_line, cursor::Up(4))?;
        handle.flush()?;

        let size = Cell::new(termion::terminal_size().unwrap_or((0, 0)));
        let end = Cell::new(false);
        let  draw_limits = Cell::new(false);

        let mut state : State = State {
            mode,
            stdout,
            handle,
            contents,
            typed_word : String::new(),
            lines,
            actual_line,
            words,
            actual_word,
            colors,
            actual_offset : 0,
            input_offset : 0,
            word_offset : 0,
            n_words : 0,
            n_correct_words : 0,
            n_lines : 0,
        };


        state.highlight_actual_word(state.colors.bg_selected)?;
        state.handle.flush()?;

        return Ok(state);
    }

    fn draw(&mut self) {
        println!("Hola que tal");
    }

    pub fn type_char(&mut self, c : char) -> io::Result<()> {
        if c != ' ' && c != '\n' {
            write!(self.handle, "{}", c)?;
            self.typed_word.push(c);
            self.next_char();
            if self.is_typed_correct() {
                self.highlight_actual_word(self.colors.bg_selected)?;
            } else {
                self.highlight_actual_word(self.colors.bg_wrong)?;
            }
        } else {
            self.next_word()?;
        }

        self.handle.flush()?;
        return Ok(());
    }

    pub fn backspace(&mut self) -> io::Result<()> {
        write!(self.handle, "{} {}", cursor::Left(1), cursor::Left(1))?;
        self.prev_char();
        if self.is_typed_correct() {
            self.highlight_actual_word(self.colors.bg_selected)?;
        } else {
            self.highlight_actual_word(self.colors.bg_wrong)?;
        }
        self.handle.flush()?;
        return Ok(());
    }

    fn is_typed_correct(&self) -> bool {
        return if self.input_offset as usize <= self.actual_word.len() {
            let mut actual_chars = self.actual_word.chars();
            self.typed_word.chars().all(|c| c == actual_chars.next().unwrap_or('\0'))
        } else {
            false
        }
    }

    fn next_char(&mut self) {
        self.actual_offset += 1;
        self.input_offset += 1;
    }

    fn prev_char(&mut self) {
        if self.actual_offset > 0 && self.input_offset > 0 {
            self.actual_offset -= 1;
            self.input_offset -= 1;
            self.typed_word.pop();
        }; 
    }

    pub fn next_word(&mut self) -> io::Result<()> {
        if self.actual_word == self.typed_word {
            self.n_correct_words += 1;
            self.color_actual_word(self.colors.fg_correct)?;
        } else {
            self.color_actual_word(self.colors.fg_wrong)?;
        }
        self.word_offset += self.actual_word.len() as u16 + 1;
        self.actual_word = self.words.next().unwrap_or("todo");
        self.actual_offset += 1;
        self.input_offset = 0;

        self.highlight_actual_word(self.colors.bg_selected)?;
        let empty = "                                                                                                                                   ";
        write!(self.handle, "{}\r", empty)?;

        self.typed_word = String::new();
        self.n_words += 1;

        return Ok(());

    }

    pub fn highlight_actual_word(&mut self, color : color::Rgb) -> io::Result<()> {
        if self.word_offset == 0 {

            write!(self.handle, "\r{}{}{}{}{}\r",
                cursor::Up(2),
                color::Bg(color),
                self.actual_word, 
                cursor::Down(2),
                color::Bg(color::Reset),
            )?;

        } else {

            write!(self.handle, "\r{}{}{}{}{}{}\r",
                cursor::Up(2),
                color::Bg(color),
                cursor::Right(self.word_offset),
                self.actual_word, 
                cursor::Down(2),
                color::Bg(color::Reset),
            )?;

        } 

        if self.input_offset > 0 {
            write!(self.handle, "{}", cursor::Right(self.input_offset))?;
        }

        return Ok(());
    }

    pub fn color_actual_word(&mut self, color : color::Rgb) -> io::Result<()> {
        if self.word_offset == 0 {

            write!(self.handle, "\r{}{}{}{}{}\r",
                cursor::Up(2),
                color::Fg(color),
                self.actual_word, 
                cursor::Down(2),
                color::Fg(color::Reset),
            )?;

        } else {

            write!(self.handle, "\r{}{}{}{}{}{}\r",
                cursor::Up(2),
                color::Fg(color),
                cursor::Right(self.word_offset),
                self.actual_word, 
                cursor::Down(2),
                color::Fg(color::Reset),
            )?;

        } 

        if self.input_offset > 0 {
            write!(self.handle, "{}", cursor::Right(self.input_offset))?;
        }

        return Ok(());
    }

    pub fn print_debug(&mut self) -> io::Result<()> {

        let empty = "                                                                                                                                   ";

        let mut debug_info : String = String::new();

        debug_info.push_str(format!("\r{}\ractual_word :       {}\n",
            empty, self.actual_word).as_str());

        debug_info.push_str(format!("\r{}\rtyped_word :        {}\n",
            empty, self.typed_word).as_str());

        debug_info.push_str(format!("\r{}\rn_correct_words :   {}\n",
            empty, self.n_correct_words).as_str());

        debug_info.push_str(format!("\r{}\rn_words :           {}\n",
            empty, self.n_words).as_str());

        debug_info.push_str(format!("\r{}\rinput_offset :      {}\n",
            empty, self.input_offset).as_str());

        debug_info.push_str(format!("\r{}\ractual_offset :     {}\n",
            empty, self.actual_offset).as_str());

        debug_info.push_str(format!("\r{}\rword_offset :       {}\n",
            empty, self.word_offset).as_str());

        if self.input_offset == 0 {
            write!(self.handle, "{}{}{}\r",
                cursor::Down(2),
                debug_info,
                cursor::Up(debug_info.lines().count() as u16 + 2))?;
        } else {
            write!(self.handle, "{}{}{}\r{}",
                cursor::Down(2),
                debug_info,
                cursor::Up(debug_info.lines().count() as u16 + 2),
                cursor::Right(self.input_offset))?;
        }
        self.handle.flush()?;
        return Ok(());
    }
}
