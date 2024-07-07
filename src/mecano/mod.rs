mod state;
mod drawing;
mod buffer;
mod word;

use std::cmp::min;
use std::time::{Duration, Instant};
use std::{io, thread};
use crossterm::event::{poll, read, Event, KeyCode};
use std::option::Option;

use crate::config::Config;
use crate::mecano::state::State;

pub struct Mecano { }

impl Mecano {
    pub fn play(config : Config) -> io::Result<()> {
        let fps = config.fps;
        let mut state = State::start(config)?;
        let mut running = false;
        let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
        let mut delta;
        let mut chrono = Instant::now();

        loop {
            state.draw()?;

            while let Ok(true) = poll(Duration::from_secs(0)) {
                running = true;
                if let None = Mecano::event_read(&mut state) {
                    return Ok(());
                }
            }

            if state.is_resized() {
                running = false;
            }

            if running {
                state.update_time(frame_duration)?;
            }

            if state.is_ended() {
                break;
            }

            delta = frame_duration - min(frame_duration, chrono.elapsed());
            thread::sleep(delta);
            chrono = Instant::now();
        }

        state.draw()?;
        loop {
            if let None = Mecano::event_read(&mut state) {
                return Ok(());
            }
        }
    }

    // TODO : Bad error managing
    fn event_read(state : &mut State) -> Option<()> {
        if let Ok(event) = read() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Esc => {
                        return None;
                    },
                    _ => {
                        let _ = state.type_key_event(key_event);
                    }
                }
            }
        }
        return Some(());
    }
}
