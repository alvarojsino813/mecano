use std::{fmt::Display, time::Duration};

use crossterm::cursor::MoveTo;

use crate::{frames::Frameable, TermUnit};

struct Input {
    input : String,
    pos : (TermUnit, TermUnit),
    size : (TermUnit, TermUnit),
}

impl Input {
    pub fn push(&mut self, c : char) {
        self.input.push(c);
    }

    pub fn pop(&mut self) {
        self.input.pop();
    }
}

impl Frameable for Input {
    fn get_size(&self) -> (TermUnit, TermUnit) {
        return self.size;
    }

    fn set_size(&mut self, size : (TermUnit, TermUnit)) {
        self.size = size;
    }

    fn get_pos(&self) -> (TermUnit, TermUnit) {
        return self.pos;
    }

    fn set_pos(&mut self, pos : (TermUnit, TermUnit)) {
        self.pos = pos;
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        self.clean_frame(f)?;

        let go_to_beginning = MoveTo(self.pos.0, self.pos.1);
        write!(f, "{go_to_beginning}")?;

        if self.size.0 == 0 {
            return Ok(());
        }

        let slice = &self.input[0..self.size.0 as usize - 1];
        write!(f, "{slice}")?;
        write!(f, "{}", self.input.chars().last().unwrap_or('\0'))?;

        return Ok(());
    }
}
