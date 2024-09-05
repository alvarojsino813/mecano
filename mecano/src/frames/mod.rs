pub mod clock_frame;
pub mod input_frame;
pub mod title_frame;
pub mod textbox;

use std::{fmt::Display, io::{stdout, Write}};

use crossterm::{cursor::{MoveDown, MoveLeft, MoveTo, MoveToColumn}, queue};

use crate::TermUnit;

/* 

─ 	━ 	│ 	┃ 	┄ 	┅ 	┆ 	┇ 	┈ 	┉ 	┊ 	┋ 	┌ 	┍ 	┎ 	┏

┐ 	┑ 	┒ 	┓ 	└ 	┕ 	┖ 	┗ 	┘ 	┙ 	┚ 	┛ 	├ 	┝ 	┞ 	┟

┠ 	┡ 	┢ 	┣ 	┤ 	┥ 	┦ 	┧ 	┨ 	┩ 	┪ 	┫ 	┬ 	┭ 	┮ 	┯

┰ 	┱ 	┲ 	┳ 	┴ 	┵ 	┶ 	┷ 	┸ 	┹ 	┺ 	┻ 	┼ 	┽ 	┾ 	┿

╀ 	╁ 	╂ 	╃ 	╄ 	╅ 	╆ 	╇ 	╈ 	╉ 	╊ 	╋ 	╌ 	╍ 	╎ 	╏

═ 	║ 	╒ 	╓ 	╔ 	╕ 	╖ 	╗ 	╘ 	╙ 	╚ 	╛ 	╜ 	╝ 	╞ 	╟

╠ 	╡ 	╢ 	╣ 	╤ 	╥ 	╦ 	╧ 	╨ 	╩ 	╪ 	╫ 	╬ 	╭ 	╮ 	╯

╰ 	╱ 	╲ 	╳ 	╴ 	╵ 	╶ 	╷ 	╸ 	╹ 	╺ 	╻ 	╼ 	╽ 	╾ 	╿

*/

type Border = (char, char, char, char, char, char, char, char, char, char, char);

const NO_BORDER : Border =      (' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ');
const LINE_BORDER : Border =    ('─', '│', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼');
const ROUND_BORDER : Border =   ('─', '│', '╭', '╮', '╰', '╯', '├', '┤', '┬', '┴', '┼');
const DASHED_BORDER : Border =  ('┈', '┊', '┌', '┐', '└', '┘', '├', '┤', '┬', '┴', '┼');

pub trait Frameable {
    fn get_size(&self) -> (TermUnit, TermUnit);
    fn set_size(&mut self, size : (TermUnit, TermUnit));

    fn get_pos(&self) -> (TermUnit, TermUnit);
    fn set_pos(&mut self, pos : (TermUnit, TermUnit));

    fn clean_frame(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty_width = " ".repeat(self.get_size().0 as usize);
        write!(f, "{}", MoveTo(self.get_pos().0, self.get_pos().1))?;
        for _ in 0..self.get_size().1 {
            write!(f,  "{empty_width}")?;
            write!(f, "{}{}",  MoveDown(1), MoveToColumn(self.get_pos().0))?;
        }
        return Ok(());
    }

    fn get_size_x(&self) -> TermUnit {
        return self.get_size().0;
    }
    fn get_size_y(&self) -> TermUnit {
        return self.get_size().1;
    }
    fn get_pos_x(&self) -> TermUnit {
        return self.get_pos().0;
    }
    fn get_pos_y(&self) -> TermUnit {
        return self.get_pos().1;
    }

    fn set_size_x(&mut self, x : TermUnit) {
        return self.set_size((x, self.get_pos_y()));
    }
    fn set_size_y(&mut self, y : TermUnit) {
        return self.set_size((self.get_pos_x(), y));
    }
    fn set_pos_x(&mut self, x : TermUnit) {
        return self.set_pos((x, self.get_pos_y()));
    }
    fn set_pos_y(&mut self, y : TermUnit) {
        return self.set_pos((self.get_pos_x(), y));
    }

}


struct Frame<T : Frameable + Display> {
    content : T,
    z_index : TermUnit,
    border : &'static Border,
}

impl<T : Frameable + Display> Frame<T> {
    pub fn new(mut content : T, 
        pos : (TermUnit, TermUnit),
        size : (TermUnit, TermUnit),
        z_index : TermUnit) -> Frame<T> {

        content.set_size(size);
        content.set_pos(pos);

        return Frame { content, z_index, border : &LINE_BORDER };
    }

    pub fn get_mut_content(&mut self) -> &mut T {
        return &mut self.content;
    }

    pub fn get_content(&self) -> &T {
        return &self.content;
    }
}

impl<T : Frameable + Display> Display for Frame<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
