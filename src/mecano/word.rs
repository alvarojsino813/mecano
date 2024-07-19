use std::{iter::Sum, slice::Iter, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CharState {
    Right,
    Wrong,
    Selected,
    Unreached,
}

#[derive(Debug)]
pub struct MecanoChar {
    pub c : char,
    pub state : CharState,
}

#[derive(Debug)]
pub struct WordPunct {
    total : usize,
    right : usize,
    wrong : usize,
    excess : usize,
    key_times : Vec<Duration>,
}

#[derive(Debug)]
pub struct MecanoWord {
    word : Vec<MecanoChar>,
    excess : String,
    select_char_idx : usize,
    state : WordState,
    punct : WordPunct,
}

#[derive(Debug)]
pub enum WordState {
    Right,
    Wrong,
    Unreached,
    Selected,
}

impl MecanoWord {
    pub fn from_str(word_str : &str) -> MecanoWord {
        let mut word : Vec<MecanoChar> = Vec::new();
        for c in word_str.chars() {
            word.push(MecanoChar {c, state : CharState::Unreached});
        }

        let mut punct = WordPunct::default();
        punct.total = word.len();
        return MecanoWord {
            word,
            excess : String::default(),
            select_char_idx : 0,
            state : WordState::Unreached,
            punct,
        }
    }

    pub fn type_char(&mut self, c : char) {
        if self.select_char_idx < self.word.len() {
            if self.word[self.select_char_idx].c == c {
                self.word[self.select_char_idx].state = CharState::Right;
                self.punct.right += 1;
            } else {
                self.word[self.select_char_idx].state = CharState::Wrong;
                self.punct.wrong += 1;
            }
            self.select_char_idx += 1;
            if self.select_char_idx < self.word.len() {
                self.word[self.select_char_idx].state = CharState::Selected;
            }
        } else {
            self.excess.push(c);
            self.punct.excess += 1;
        }
    }

    pub fn push_duration(&mut self, d : Duration) {
        self.punct.key_times.push(d);
    }

    pub fn delete(&mut self) {
        if self.punct.key_times.len() > 2 {
            self.punct.key_times.swap_remove(self.punct.key_times.len() - 2);
        } else {
            self.punct.key_times.pop();
        }
        if self.punct.excess > 0 {
            self.excess.pop();
            self.punct.excess -= 1;
        } else if self.select_char_idx > 0 {
            self.word
                .get_mut(self.select_char_idx - 1)
                .map(|c| { 
                    // Excess??
                    if c.state == CharState::Right {
                        self.punct.right -= 1;
                    } else {
                        self.punct.wrong -= 1;
                    }
                    c.state = CharState::Unreached;
                });
            self.select_char_idx -= 1;
            self.word[self.select_char_idx].state = CharState::Selected;
        }
    }

    pub fn total_chars(&self) -> usize {
        return self.word.iter().count() + self.punct.excess;
    }

    pub fn select(&mut self) {
        self.state = WordState::Selected;
        self.word[0].state = CharState::Selected;
    }

    pub fn unselect(&mut self) {
        if self.iter().all(|c| c.state == CharState::Right) {
            self.state = WordState::Right;
        } else {
            self.state = WordState::Wrong;
        }

        for c in self.word.iter_mut() {
            if c.state == CharState::Selected {
                c.state = CharState::Unreached;
            }
        }
    }

    pub fn get_punct(&self) -> &WordPunct {
        return &self.punct;
    }

    pub fn iter(&self) -> Iter<MecanoChar> {
        return self.word.iter();
    }

    pub fn excess(&self) -> &str {
        return &self.excess;
    }
}

impl WordPunct {
    pub fn default() -> WordPunct {
        return WordPunct {
            total : 0,
            right : 0,
            wrong : 0,
            excess : 0,
            key_times : vec!(),
        }
    }
}
