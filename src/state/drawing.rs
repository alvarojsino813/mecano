use std::io;

use crossterm::{cursor::{MoveTo, MoveToColumn, MoveToNextLine}, queue, terminal::{Clear, ClearType}};

fn draw_box(f: &mut std::fmt::Formatter<'_>) -> io::Result<()> {
    let mut string = String::from("┏");
    string.push_str("━".repeat(self.box_info.size.0 as usize - 2).as_str());
    string.push('┓');
    queue!(f, MoveTo(0, 0))?;
    write!(f, "{}", string)?;

    for _row in 2 .. self.box_info.size.0 {
        write!(f, "┃")?;
        queue!(f, MoveToColumn(self.box_info.size.0))?;
        write!(f, "┃")?;
        queue!(f, MoveToNextLine(1))?;
    }

    let mut string = String::from("┗");
    string.push_str("━".repeat(self.box_info.size.0 as usize - 2).as_str());
    string.push('┛');
    write!(f, "{}", string)?;
    return Ok(());
}


fn print_empty_width(f: &mut std::fmt::Formatter<'_>) -> io::Result<()> {
    queue!(f, MoveToColumn(2))?; 
    let empty = " ".repeat(self.box_info.size.0 as usize - 3);
    write!(f, "{}", empty)?;
    Ok(())
}

fn draw_width_warning(f: &mut std::fmt::Formatter<'_>) -> io::Result<()> {
    queue!(f, MoveTo(0, 0), Clear(ClearType::All));
    write!(f, "\rT\n\rO\n\rO\n\n\rN\n\rA\n\rR\n\rR\n\rO\n\rW\n\r");
    return Ok(());
}
