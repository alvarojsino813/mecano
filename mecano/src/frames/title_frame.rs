use std::{cmp::min, fmt::Display, time::Duration};

use crossterm::{cursor::{MoveDown, MoveTo, MoveToColumn}, queue};

use crate::{frames::Frameable, TermUnit};

const TITLE : &'static [&'static str] = &[
    "███╗   ███╗███████╗ ██████╗ █████╗ ███╗   ██╗ ██████╗ ",
    "████╗ ████║██╔════╝██╔════╝██╔══██╗████╗  ██║██╔═══██╗",
    "██╔████╔██║█████╗  ██║     ███████║██╔██╗ ██║██║   ██║",
    "██║╚██╔╝██║██╔══╝  ██║     ██╔══██║██║╚██╗██║██║   ██║",
    "██║ ╚═╝ ██║███████╗╚██████╗██║  ██║██║ ╚████║╚██████╔╝",
    "╚═╝     ╚═╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ",
];

pub struct Title {
    title : &'static [&'static str],
    pos : (TermUnit, TermUnit),
    size : (TermUnit, TermUnit),
}

impl Title {
    pub fn new(pos : (TermUnit, TermUnit), size : (TermUnit, TermUnit)) -> Title {
        return Title {
            title : TITLE,
            pos,
            size,
        };
    }

}

impl Frameable for Title {
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

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        self.clean_frame(f)?;

        let title_width = self.title.first().unwrap().chars().count() as u16;
        let title_height = self.title.len() as u16;

        if title_width > self.get_size().0 || title_height > self.get_size().1 {
            return Ok(());
        }

        let x = self.get_pos().0 + (self.get_size().0 - title_width as u16) / 2;
        let y = self.get_pos().1 + (self.get_size().1 - title_height as u16) / 2;

        let go_to_beginning = MoveTo(x, y);
        write!(f, "{go_to_beginning}")?;

        let min_height = min(title_height, self.get_size().1) as usize;
        let trimmed_title = &self.title[0..min_height];

        let min_width = min(title_width, self.get_size().0) as usize;
        for line in trimmed_title {
            let trimmed_line : String = line.chars().take(min_width).collect();
            write!(f, "{trimmed_line}{}{}",
                MoveDown(1),
                MoveToColumn(x))?;
        }

        if self.size.0 == 0 {
            return Ok(());
        }

        return Ok(());
    }
}
