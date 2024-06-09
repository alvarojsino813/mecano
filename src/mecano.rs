use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::Mutex;
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
    actual_time : Duration,
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
                let mut actual_second = 0;
                while !end_clone.load(Relaxed) {
                    let time = Instant::now();

                    let actual_size = crossterm::terminal::size().unwrap();
                    if state_clone.lock().unwrap().get_size()
                        !=
                        actual_size {
                        let _ = state_clone.lock().unwrap().draw();
                    }

                    if chrono.elapsed().as_secs()
                    > 
                    Duration::from_secs(actual_second).as_secs()
                    {
                        actual_second = chrono.elapsed().as_secs();
                        let _ = state_clone.lock().unwrap().sub_sec();
                    }

                    let delta_time = time.elapsed();
                    thread::sleep(Duration::from_millis(1000 / fps) - delta_time);
                }
                let _ = state_clone.lock().unwrap().draw();
            }));


        Ok(Mecano {
            state,
            end,
            join_handle,
            actual_time : Duration::from_secs(0), 
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
        println!("");
    }
}
