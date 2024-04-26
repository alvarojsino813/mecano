use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{rc::Rc, sync::Mutex};
use std::{io, thread};
use crossterm::event::KeyEvent;
use std::thread::{spawn, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::option::Option;

use crate::{config::Config, state::State};

pub struct Mecano {
    state : Arc<Mutex<State>>,
    end : Arc<AtomicBool>,
    join_handle : Option<JoinHandle<()>>,
}

impl Mecano {

    pub fn start(path_to_dict : &str) -> io::Result<Mecano> {
        let end = Arc::new(AtomicBool::new(false));

        let state = Arc::new(Mutex::new(State::start(path_to_dict)?));
        let state_clone = Arc::clone(&state);

        let end_clone = Arc::clone(&end);
        let fps = 60;
        
        let join_handle = Some(spawn(move ||
            {
                let chrono = Instant::now();
                while !end_clone.load(Relaxed) {
                    let time = Instant::now();

                    let actual_size = crossterm::terminal::size().unwrap();
                    if state_clone.lock().unwrap().get_size()
                        !=
                        actual_size {
                        let _ = state_clone.lock().unwrap().draw_dict_mode();
                    }

                    // Timer to finish

                    if chrono.elapsed() > Duration::from_secs(1) {
                        let _ = state_clone.lock().unwrap().draw_punct();
                        let _ = state_clone.lock().unwrap().end();
                    }

                    let delta_time = time.elapsed();
                    thread::sleep(Duration::from_millis(1000 / fps) - delta_time);
                }
            }));


        Ok(Mecano {
            state,
            end,
            join_handle,
        })
    }

    pub fn type_key_event(&mut self, key : KeyEvent) -> io::Result<()> {
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
    }
}
