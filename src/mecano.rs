use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{rc::Rc, sync::Mutex};
use std::{io, thread};
use termion::event::Key;
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
                while !end_clone.load(Relaxed) {
                    let time = Instant::now();

                    let actual_size = termion::terminal_size().unwrap();
                    if state_clone.lock().unwrap().get_size()
                        !=
                        actual_size {
                        let _ = state_clone.lock().unwrap().draw();
                    }

                    // Timer to finish


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

    pub fn type_key(&mut self, key : Key) -> io::Result<()> {
        return self.state.lock().unwrap().type_key(key);
    }
}

impl Drop for Mecano {
    fn drop(&mut self) {
        self.end.store(true, Relaxed);
        if self.join_handle.is_some() {
            let _ = self.join_handle.take().unwrap().join();
        }
    }
}
