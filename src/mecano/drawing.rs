use std::io::{self, stdout, Write};

use crossterm::{cursor::{MoveDown, MoveTo, MoveToColumn, MoveToNextLine}, queue, terminal::{Clear, ClearType}};

pub struct BoxInfo {
    pub left_padding : u16,
    pub top_padding : u16,
    pub width : u16,
    pub size : (u16, u16),
}

impl BoxInfo {
    pub fn centered() -> Result<BoxInfo, ()> {
        let size = crossterm::terminal::size().unwrap();
        let width = 70;
        if size.0 < width + 1 {return Err(())}
        let left_padding = (size.0 - width) / 2;
        let top_padding = (size.1 - 4) / 2;

        Ok(BoxInfo {
            left_padding,
            top_padding,
            width,
            size,
        })
    }

    pub fn initial_pos(&self) -> (u16, u16) {
        return (self.left_padding, self.top_padding)
    }
}

pub fn print_empty_width(offset : u16, box_width : u16) -> io::Result<()> {
    queue!(stdout(), MoveToColumn(offset))?; 
    let empty = " ".repeat(box_width as usize);
    write!(stdout(), "{}", empty)?;
    Ok(())
}

pub fn draw_width_warning() -> io::Result<()> {
    queue!(stdout(), MoveTo(0, 0), Clear(ClearType::All))?;
    write!(stdout(), "\rT\n\rO\n\rO\n\n\rN\n\rA\n\rR\n\rR\n\rO\n\rW\n\r")?;
    return Ok(());
}


pub fn draw_box(position : (u16, u16), box_size : (u16, u16)) -> io::Result<()> {
    let mut string = String::from("┏");
    string.push_str("━".repeat(box_size.0 as usize - 2).as_str());
    string.push('┓');
    queue!(stdout(), MoveTo(position.0, position.1))?;
    write!(stdout(), "{}", string)?;
    queue!(stdout(), MoveToColumn(position.0))?;
    queue!(stdout(), MoveToNextLine(1))?;

    for _row in 2 .. box_size.1 {
        queue!(stdout(), MoveToColumn(position.0))?;
        write!(stdout(), "┃")?;
        queue!(stdout(), MoveToColumn(position.0 + box_size.0 - 1))?;
        write!(stdout(), "┃")?;
        queue!(stdout(), MoveDown(1))?;
    }

    let mut string = String::from("┗");
    string.push_str("━".repeat(box_size.0 as usize - 2).as_str());
    string.push('┛');
    queue!(stdout(), MoveToColumn(position.0))?;
    write!(stdout(), "{}", string)?;
    return Ok(());
}
