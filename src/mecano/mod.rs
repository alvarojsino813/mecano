mod state;
mod line;
mod drawing;

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use std::{io, thread};
use crossterm::event::KeyEvent;
use std::thread::{spawn, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::option::Option;

use crate::config::Config;
use crate::mecano::state::State;

pub struct Mecano {
    state : Arc<Mutex<State>>,
    end : Arc<AtomicBool>,
    join_handle : Option<JoinHandle<()>>,
    running : Arc<AtomicBool>,
    first_press : bool,
}

impl Mecano {
    pub fn start(config : Config) -> io::Result<Mecano> {
        let end = Arc::new(AtomicBool::new(false));
        let running = Arc::new(AtomicBool::new(false));

        let state = State::start(config)?;
        let state = Arc::new(Mutex::new(state));
        let state_clone = Arc::clone(&state);

        let end_clone = Arc::clone(&end);
        let running_clone = Arc::clone(&running);
        let fps = 60;
        
        let join_handle = Some(spawn(move ||
            {
                let mut chrono = Instant::now();
                let mut sec_left = Duration::from_millis(1000);
                while !end_clone.load(Relaxed) {
                    let delta_time = Instant::now();

                    let actual_size = crossterm::terminal::size().unwrap();
                    if state_clone.lock().unwrap().get_size() != actual_size {
                        running_clone.store(false, Relaxed);
                        sec_left = chrono.elapsed();
                        let _ = state_clone.lock().unwrap().draw();
                    }

                    if !running_clone.load(Relaxed) {
                        chrono = Instant::now();
                    }

                    if chrono.elapsed() > sec_left &&
                    running_clone.load(Relaxed) {
                        let _ = state_clone.lock().unwrap().sub_sec();
                        chrono = Instant::now();
                        sec_left = Duration::from_millis(1000);
                    }

                    let delta_time = delta_time.elapsed();
                    if delta_time < Duration::from_millis(1000 / fps) {
                        thread::sleep(Duration::from_millis(1000 / fps) - delta_time);
                    }
                }
                let _ = state_clone.lock().unwrap().draw();
            }));


        Ok(Mecano {
            state,
            end,
            join_handle,
            running,
            first_press : false,
        })
    }

    pub fn type_key_event(&mut self, key : KeyEvent) -> io::Result<()> {
        self.running.store(true, Relaxed);
        if !self.first_press {
            self.state.lock().unwrap().sub_sec();
            self.first_press = true;
        }
        return self.state.lock().unwrap().type_key_event(key);
    }

    pub fn is_ended(&self) -> bool {
        return self.end.load(Relaxed);
    }
}

impl Drop for Mecano {
    fn drop(&mut self) {
        self.end.store(true, Relaxed);
        if let Some(handle) = self.join_handle.take() {
            let _ = handle.join();
        }
        println!("");
    }
}
