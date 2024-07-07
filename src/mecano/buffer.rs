use std::{collections::VecDeque, fmt::Formatter};
use core::fmt::Display;

use crossterm::{cursor::{MoveDown, MoveToColumn}, style::{Color, SetForegroundColor}};

use crate::{config::ConfigBuffer, mecano::word::CharState, modes::Mode};

use super::word::MecanoWord;

pub struct MecanoBuffer {
    mecano_words : VecDeque<MecanoWord>,
    words_source : Mode,
    idx_typing_word : usize,
    idx_print_offset : usize,
    n_typed_line_chars : usize,
    n_total_chars_to_show : usize,
    config : ConfigBuffer,
    size : (u16, u16),
    column : u16,
}

impl MecanoBuffer {

    pub fn new(words_source : Mode, config : ConfigBuffer, size : (u16, u16)) -> MecanoBuffer {
        let buffer : VecDeque<MecanoWord> = VecDeque::new();

        let mut mecano_buffer =  MecanoBuffer {
            mecano_words: buffer,
            words_source,
            idx_typing_word : 0,
            idx_print_offset : 0,
            n_typed_line_chars : 0,
            n_total_chars_to_show : 0,
            config,
            size, 
            column : 0,
        };

        mecano_buffer.set_size(size);
        return mecano_buffer;
    }

    pub fn type_char(&mut self, c : char) {
        if c.is_whitespace() {
            self.next_word();
        } else {
            self.mecano_words[self.idx_typing_word].type_char(c);
        }
    }

    fn next_word(&mut self) {
        let n_word_chars = self.mecano_words[self.idx_typing_word].iter().count();
        self.n_total_chars_to_show -= n_word_chars + 1;
        self.n_typed_line_chars += 
            self.mecano_words[self.idx_typing_word].total_chars() + 1;
        self.idx_typing_word += 1;
        self.mecano_words[self.idx_typing_word].select();

        // Checks if next line
        if self.size.1 as usize 
        <= self.n_typed_line_chars 
        + self.mecano_words[self.idx_typing_word].total_chars() {
            self.idx_print_offset = self.idx_typing_word;
            self.n_typed_line_chars = 0;
        }
    }

    // Actualiza n_typed_line_chars
    pub fn backspace(&mut self) {
        self.mecano_words[self.idx_typing_word].delete();
    }

    fn complete_size(&mut self) {
        while self.n_total_chars_to_show < self.size.0 as usize * self.size.1 as usize {
            eprintln!("Added word : {}", self.mecano_words.len());
            let new_word = self.words_source.yield_word();
            self.n_total_chars_to_show += new_word.chars().count() + 1;
            self.mecano_words.push_back(MecanoWord::from_str(new_word));
        }
    }

    // Actualizar print_offset
    pub fn set_size(&mut self, size : (u16, u16)) {
        self.size = size;
        self.complete_size();
    }

    pub fn set_column(&mut self, column : u16) {
        self.column = column;

    }
}

// TODO: Tener en cuenta width y lines_to_show
impl Display for MecanoBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut width = 0;
        let mut lenght = 0;
        let column = self.column;
        for word in self.mecano_words.iter().skip(self.idx_print_offset) {
            width += word.total_chars() + 1;
            if width > self.size.1 as usize {
                lenght += 1;
                width = word.total_chars() + 1;
                write!(f, "{}{}", MoveToColumn(column), MoveDown(1))?;
            }
            if lenght >= self.size.0 as usize {
                break;
            }
            for m_c in word.iter() {
                let color =
                match m_c.state {
                    CharState::Correct => self.config.correct,
                    CharState::Wrong => self.config.wrong,
                    CharState::Selected => self.config.selected,
                    CharState::Unreached => Color::Reset,
                };
                write!(f, "{}{}", SetForegroundColor(color), m_c.c)?;
            }
            write!(f, "{}{}{}"
                ,SetForegroundColor(self.config.wrong)
                ,word.excess()
                ,SetForegroundColor(Color::Reset))?;
            write!(f, " ")?;
        }
        return Ok(());
    }
}

#[cfg(test)]

mod test {

    use crate::config::ConfigBuffer;

    use super::MecanoBuffer;


    #[test]
    fn mecano_buffer_display() {
        let words = vec!["hola".to_string(), "adios".to_string()];

        let config = ConfigBuffer::_default();

        let size = (5, 5);

        let buffer = MecanoBuffer::new(words, config, size);

        println!("buffer : {}", buffer);
    }
}
