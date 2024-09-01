use std::{
    collections::HashMap,
    fmt::Display,
    time::Duration
};

use crossterm::cursor::{MoveDown, MoveTo};

use crate::{Count, TermUnit};

const SECS_PER_MIN : f64 = 60.0;
const CHARS_PER_WORD : f64 = 5.0;

#[derive(Debug, Clone)]
pub struct Punct {
    chars_right : Count,
    chars_wrong : Count,
    chars_extra : Count,
    chars_missed : Count,
    stats : Vec<Stats>,
    mapped_key_presses : HashMap<char, KeyPress>,
    total_time : Duration,
    size : (TermUnit, TermUnit),
    pos : (TermUnit, TermUnit),
}

#[derive(Debug, Clone)]
struct Stats {
    wpm : f64,
    raw : f64,
    acc : f64,
}

impl Punct {
    pub fn new() -> Punct {
        return Punct {
            chars_right : 0,
            chars_wrong : 0,
            chars_extra : 0,
            chars_missed : 0,
            stats : Vec::new(),
            mapped_key_presses : HashMap::new(),
            total_time : Duration::ZERO,
            size : (0, 0),
            pos : (0, 0),
        }
    }

    // Test purpouse
    pub fn get_raw_info(&self) -> 
    (Count, Count, Count, Count, f64, f64, f64) {
        let final_stats = self.stats.iter().last().unwrap();
        let raw = final_stats.raw;
        let wpm = final_stats.wpm;
        let acc = final_stats.acc;
        (self.chars_right, self.chars_wrong, self.chars_extra, self.chars_missed,
             raw, wpm, acc)
    }

    pub fn push_punct_word(&mut self, punct_word : &PunctWord) {
        self.chars_right += punct_word.right;            
        self.chars_extra += punct_word.extra;            
        self.chars_wrong += punct_word.wrong;            
        self.chars_missed += punct_word.missed;            
        for key_press in &punct_word.key_presses {
            let key_press = key_press.clone();
            self.mapped_key_presses.insert(key_press.pressed, key_press);
            self.total_time += key_press.dur;
        }
        self.stats.push(self.calc_stats());
    }

    fn calc_stats(&self) -> Stats {
        let wpm = (self.chars_right) as f64 / 
        self.total_time.as_secs_f64() * SECS_PER_MIN / CHARS_PER_WORD;
        
        let raw = (self.chars_right + self.chars_wrong) as f64 /
        self.total_time.as_secs_f64() * SECS_PER_MIN / CHARS_PER_WORD;
        
        let acc = self.chars_right as f64 /
        (self.chars_right + self.chars_wrong) as f64;

        return Stats {
            wpm,
            raw,
            acc,
        }
    }

    pub fn set_size(&mut self, size : (TermUnit, TermUnit)) {
        self.size = size;
    }

    pub fn set_pos(&mut self, pos : (TermUnit, TermUnit)) {
        self.pos = pos;
    }
}

// REFACTOR
impl Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let top = self.size.0 / 2 - 11 / 2;
        let left = self.size.1 / 2 - 3 / 2;
        let go_to_top_left = MoveTo(top, left);
        let go_down = MoveDown(1);

        write!(f, "{go_to_top_left}")?;

        let raw = self.stats.last().unwrap().raw;
        write!(f, "RAW  {raw:.2}")?;
        write!(f, "{go_to_top_left}{go_down}")?;

        let wpm = self.stats.last().unwrap().wpm;
        write!(f, "WPM  {wpm:.2}")?;
        write!(f, "{go_to_top_left}{go_down}{go_down}")?;

        let acc = self.stats.last().unwrap().acc * 100.0;
        write!(f, "ACC  {acc:.2}%")?;
        write!(f, "{go_to_top_left}{go_down}{go_down}{go_down}")?;

        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub struct PunctWord {
    total : Count,
    right : Count,
    wrong : Count,
    extra : Count,
    missed : Count,
    key_presses : Vec<KeyPress>,
    buffer_dur : Duration,
}

impl PunctWord {
    pub fn new(total : Count) -> PunctWord {
        return PunctWord {
            total,
            right : 0,
            wrong : 0,
            extra : 0,
            missed : 0,
            key_presses : Vec::new(),
            buffer_dur : Duration::ZERO,
        }
    }

    pub fn add_key_press(&mut self, k : KeyPress) {
        if k.pressed == ' ' {
            // Spaces not counting for punct
        } else if k.pressed == k.aim {
            self.right += 1;
        } else if k.aim == '\0' {
            self.extra += 1;
        } else {
            self.wrong += 1;
        }
        let mut key_press = k;
        key_press.dur += self.buffer_dur;
        self.buffer_dur = Duration::ZERO;
        self.key_presses.push(key_press);
    }

    pub fn sub_key_press(&mut self) {
        if let Some(k) = self.key_presses.last() {
            self.buffer_dur += k.dur;
        } 
        self.key_presses.pop();
    }

    pub fn get_punct(&self) -> (Count, Count, Count, Count, Count) {
        return (self.total, self.right, self.wrong, self.extra, self.missed)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyPress {
    aim : char,
    pressed : char,
    dur : Duration,
}

impl KeyPress {
    pub fn new(aim : char, pressed : char, dur : Duration) -> KeyPress {
        return KeyPress {
            aim,
            pressed,
            dur,
        }
    }
}
