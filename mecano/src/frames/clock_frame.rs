use std::{fmt::Display, time::Duration};

use crossterm::cursor::MoveTo;

use crate::{frames::Frameable, TermUnit};

struct Clock {
    time : Duration,
    pos : (TermUnit, TermUnit),
    size : (TermUnit, TermUnit),
}

impl Clock {
    pub fn add_time(&mut self, time : Duration) {
        self.time += time;
    }

    pub fn sub_time(&mut self, time : Duration) {
        self.time -= std::cmp::min(time, self.time);
    }

    pub fn set_time(&mut self, time : Duration) {
        self.time = time;
    }
}

impl Frameable for Clock {
    fn get_size(&self) -> (TermUnit, TermUnit) {
        return self.size;
    }

    fn set_size(&mut self, size : (TermUnit, TermUnit)) {
        self.size = size;
    }

    fn get_pos(&self) -> (TermUnit, TermUnit) {
        return self.pos;
    }

    fn set_pos(&mut self, pos : (TermUnit, TermUnit)) {
        self.pos = pos;
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        self.clean_frame(f)?;

        let go_to_beginning = MoveTo(self.pos.0, self.pos.1);
        write!(f, "{go_to_beginning}")?;

        let secs = self.time.as_secs() % 60;
        let mins = self.time.as_secs() / 60;
        if self.get_size().0 > 5 {
            write!(f, "{mins:0>2}:{secs:0>2}")?;
        }
        return Ok(());
    }
}
