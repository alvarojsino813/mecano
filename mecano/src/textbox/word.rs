use std::{slice::Iter, time::Duration};

use super::{Count, Idx, TermUnit};


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Right,
    Wrong,
    Unreached,
    Selected,
}

#[derive(Debug, Clone)]
pub struct StatefulChar {
    pub c : char,
    pub state : State,
}

impl Default for StatefulChar {
    fn default() -> Self {
        return StatefulChar {
            c : '\0',
            state : State::Unreached,
        }
    }
}

impl Default for &StatefulChar {
    fn default() -> Self {
        return &StatefulChar {
            c : '\0',
            state : State::Unreached,
        }
    }
}

impl<'a> FromIterator<&'a StatefulChar> for Word {
    fn from_iter<T: IntoIterator<Item = &'a StatefulChar>>(iter: T) -> Word {
        let iterator = iter.into_iter().cloned();
        let (lower_bound, _) = iterator.size_hint();
        let mut collected = Vec::with_capacity(lower_bound);
        for c in iterator {
            collected.push(c);
        }
        return Word::from_stateful_chars(collected);
    }
}

impl FromIterator<StatefulChar> for Word {
    fn from_iter<T: IntoIterator<Item = StatefulChar>>(iter: T) -> Word {
        let iterator = iter.into_iter();
        let (lower_bound, _) = iterator.size_hint();
        let mut collected = Vec::with_capacity(lower_bound);
        for c in iterator {
            collected.push(c);
        }
        return Word::from_stateful_chars(collected);
    }
}

#[derive(Debug)]
pub struct PunctWord {
    pub total : Count,
    pub right : Count,
    pub wrong : Count,
    pub extra : Count,
    pub missed : Count,
    pub state : State,
    pub key_times : Vec<Duration>,
}

#[derive(Debug)]
pub struct Word {
    word : Vec<StatefulChar>,
    extra : String,
    selected_char : Idx,
    punct : PunctWord,
}


impl Word {
    pub fn from_str(word_str : &str) -> Word {
        let mut word : Vec<StatefulChar> = Vec::new();
        for c in word_str.chars() {
            word.push(StatefulChar {c, state : State::Unreached});
        }

        let mut punct = PunctWord::new();
        punct.total = word.len() as Count;
        return Word {
            word,
            extra : String::default(),
            selected_char : 0,
            punct,
        }
    }

    pub fn from_stateful_chars(stateful_chars : Vec<StatefulChar>) -> Word {
        let word = stateful_chars;
        let mut punct = PunctWord::new();
        punct.total = word.len() as Count;
        return Word {
            word,
            extra : String::default(),
            selected_char : 0,
            punct,
        }

    }

    pub fn type_char(&mut self, c : char) {
        if self.selected_char < self.word.len() {
            if self.word[self.selected_char].c == c {
                self.word[self.selected_char].state = State::Right;
                self.punct.right += 1;
            } else {
                self.word[self.selected_char].state = State::Wrong;
                self.punct.wrong += 1;
            }
            self.selected_char += 1;
            if self.selected_char < self.word.len() {
                self.word[self.selected_char].state = State::Selected;
            }
        } else {
            self.extra.push(c);
            self.punct.extra += 1;
        }
    }

    pub fn push_duration(&mut self, d : Duration) {
        self.punct.key_times.push(d);
    }

    pub fn pop(&mut self) {
        if self.punct.key_times.len() > 2 {
            let len = self.punct.key_times.len();
            let dur = self.punct.key_times[len - 1] 
                + self.punct.key_times[len - 2];
            self.punct.key_times[len - 1] = dur;
            self.punct.key_times.swap_remove(self.punct.key_times.len() - 2);
        } else {
            self.punct.key_times.pop();
        }
        if self.punct.extra > 0 {
            self.extra.pop();
            self.punct.extra -= 1;
        } else if self.selected_char > 0 {
            let c = &self.word[self.selected_char - 1];
            if c.state == State::Right {
                self.punct.right -= 1;
            } else if c.state == State::Wrong {
                self.punct.wrong -= 1;
            }
            if self.selected_char < self.word.len() {
                self.word[self.selected_char].state = State::Unreached;
            }
            self.selected_char -= 1;
            self.word[self.selected_char].state = State::Selected;
        } 
    }

    pub fn n_chars_and_extra(&self) -> TermUnit {
        return self.word.iter().count() as TermUnit 
        + self.punct.extra as TermUnit;
    }

    pub fn n_chars(&self) -> TermUnit {
        return self.word.iter().count() as TermUnit 
    }

    pub fn n_extra(&self) -> TermUnit {
        return self.extra().chars().count() as TermUnit;
    }

    pub fn select(&mut self) {
        self.punct.state = State::Selected;
        self.word[0].state = State::Selected;
    }

    pub fn unselect(&mut self) {
        if self.chars().all(|c| c.state == State::Right) {
            self.punct.state = State::Right;
        } else {
            self.punct.state = State::Wrong;
        }

        for c in self.word.iter_mut() {
            if c.state == State::Unreached {
                self.punct.missed += 1;
            }
            if c.state == State::Selected {
                c.state = State::Unreached;
                self.punct.missed += 1;
            }
        }
    }

    pub fn is_selected(&self) -> bool { 
        return self.punct.state == State::Selected
    }

    pub fn get_punct(&self) -> &PunctWord {
        return &self.punct;
    }

    pub fn chars(&self) -> Iter<StatefulChar> {
        return self.word.iter();
    }

    pub fn extra(&self) -> &str {
        return &self.extra;
    }
}

impl PunctWord {
    pub fn new() -> PunctWord {
        return PunctWord {
            total : 0,
            right : 0,
            wrong : 0,
            extra : 0,
            missed : 0,
            state : State::Unreached,
            key_times : vec!(),
        }
    }
}
