use std::io::{self, Read};
use std::fs::File;
use termion::input::TermRead;
use termion::event::Key;
use termion;
use state::State;

mod state;
mod config;
mod mecano;
mod dictionary;

fn main() -> io::Result<()> {

    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut state = State::start(contents.as_str()).unwrap();
    let stdin = io::stdin();

    for key in stdin.keys() {
        if let Ok(Key::Esc) = key {
            break;
        } else {
            let _ = state.type_key(key.unwrap());
        }


    }

    println!("{}\r", termion::clear::All);
    return Ok(());
}
