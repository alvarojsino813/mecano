use std::{
    slice::Iter,
    time::Duration
};

use crate::punctuation::{KeyPress, PunctWord};

use super::{Count, Idx, State, TermUnit};

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

        let punct = PunctWord::new(word.len() as Count + 1);
        return Word {
            word,
            extra : String::default(),
            selected_char : 0,
            punct,
        }
    }

    pub fn from_stateful_chars(stateful_chars : Vec<StatefulChar>) -> Word {
        let punct = PunctWord::new(stateful_chars.len() as Count);
        let word = stateful_chars;
        return Word {
            word,
            extra : String::default(),
            selected_char : 0,
            punct,
        }
    }

    pub fn type_char(&mut self, c : char, dur : Duration) {
        // Next word
        if c == ' ' {
            self.punct.add_key_press(
                KeyPress::new(c, c, dur)
            );

        // No extra chars
        } else if self.selected_char < self.word.len() {
            if self.word[self.selected_char].c == c {
                self.word[self.selected_char].state = State::Right;
            } else {
                self.word[self.selected_char].state = State::Wrong;
            }
            self.selected_char += 1;
            if self.selected_char < self.word.len() {
                self.word[self.selected_char].state = State::Selected;
            }

            self.punct.add_key_press(
                KeyPress::new(self.word[self.selected_char - 1].c, c, dur)
            );

        // Extra chars
        } else {
            self.extra.push(c);
            self.punct.add_key_press(
                KeyPress::new('\0', c, dur)
            );
        }
    }

    pub fn pop(&mut self) {
        if self.extra.chars().count() > 0 {
            self.extra.pop();
        } else if self.selected_char > 0 {
            if self.selected_char < self.word.len() {
                self.word[self.selected_char].state = State::Unreached;
            }
            self.selected_char -= 1;
            self.word[self.selected_char].state = State::Selected;
        } 
        self.punct.sub_key_press();
    }

    pub fn n_chars_and_extra(&self) -> TermUnit {
        return self.n_chars() + self.n_extra();
    }

    pub fn n_chars(&self) -> TermUnit {
        return self.word.iter().count() as TermUnit 
    }

    pub fn n_extra(&self) -> TermUnit {
        return self.extra().chars().count() as TermUnit;
    }

    pub fn select(&mut self) {
        self.word[0].state = State::Selected;
    }

    pub fn unselect(&mut self) {
        for c in self.word.iter_mut() {
            if c.state == State::Selected {
                c.state = State::Unreached;
            }
        }
    }

    pub fn is_selected(&self) -> bool { 
        return self.word.iter().any(|s_c| s_c.state == State::Selected);
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


