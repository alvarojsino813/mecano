use std::io;
use std::time::Duration;
use crossterm::event::poll;
use crossterm::event::Event;
use crossterm::event::read;

use crossterm::event::KeyCode;
use mecano::Mecano;

mod config;
mod mecano;
mod dictionary;

fn main() -> io::Result<()> {

    let mut mecano = Mecano::start("100_spanish")?;

    while !mecano.is_ended() {
        if let Ok(true) = poll(Duration::from_secs(0)) { break; }
        if let Ok(event) = read() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Esc => {
                        break;
                    },
                    _ => {
                        mecano.type_key_event(key_event)?
                    }
                }
            }
        }
    }

    return Ok(());
}
