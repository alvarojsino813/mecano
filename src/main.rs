use std::io;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion;
use mecano::Mecano;

mod state;
mod config;
mod mecano;
mod dictionary;

fn main() -> io::Result<()> {

    let stdin = io::stdin();
    let mut mecano = Mecano::start("100_spanish").unwrap();

    for key in stdin.keys() {
        if let Ok(Key::Esc) = key {
            break;
        } else {
            mecano.type_key(key.unwrap())?;
        }
    }

    println!("{}\r", termion::clear::All);
    return Ok(());
}
