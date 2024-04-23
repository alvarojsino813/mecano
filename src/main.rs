use std::io::{self, Read};
use std::fs::File;
use termion::input::TermRead;
use termion::event::Key;
use termion;
use state::State;

mod state;
mod dictionary;

fn main() -> io::Result<()> {

    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", termion::clear::All);
    let mut state = State::start(contents.as_str()).unwrap();
    let stdin = io::stdin();
    state.print_debug()?;

    for key in stdin.keys() {
        match key? {
            Key::Char(c) => {
                state.type_char(c)?;
            },

            Key::Esc => break,

            Key::Insert =>  {
                state.next_word()?;
            }

            Key::Backspace => {
                state.backspace()?;
            }

            _ => (),
        }

        state.print_debug()?;

    }

    println!("\n\r");
    println!("\n\r");
    println!("\n\r");
    println!("\n\r");
    println!("\n\r");
    return Ok(());
}
