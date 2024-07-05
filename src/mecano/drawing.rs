use std::io::{self, stdout, Write};

use crossterm::{cursor::{MoveDown, MoveTo, MoveToColumn, MoveToNextLine}, queue, terminal::{Clear, ClearType}};

pub struct BoxInfo {
    pub left_padding : u16,
    pub top_padding : u16,
    pub width : u16,
    pub size : (u16, u16),
}

impl BoxInfo {
    pub fn centered(box_width : u16, size : (u16, u16)) -> Result<BoxInfo, ()> {
        if size.0 < box_width + 1 || size.1 < 9 {return Err(())}
        let left_padding = (size.0 - box_width) / 2;
        let top_padding = (size.1 - 4) / 2;

        Ok(BoxInfo {
            left_padding,
            top_padding,
            width : box_width,
            size,
        })
    }

    pub fn default() -> BoxInfo {
        let size = crossterm::terminal::size().unwrap();

        BoxInfo {
            left_padding : 0,
            top_padding : 0,
            width : 0,
            size,
        }
    }
}

pub fn print_empty_width(offset : u16, box_width : u16) -> io::Result<()> {
    queue!(stdout(), MoveToColumn(offset))?; 
    let empty = " ".repeat(box_width as usize);
    write!(stdout(), "{}", empty)?;
    Ok(())
}

pub fn draw_too_narrow() -> io::Result<()> {
    queue!(stdout(), MoveTo(0, 0), Clear(ClearType::All))?;
    write!(stdout(), "\rTOO NARROW. RESIZE.")?;
    stdout().flush()?;
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
