use std::slice::{Iter, IterMut};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CharState {
    Correct,
    Wrong,
    Selected,
    Unreached,
}

#[derive(Debug)]
pub struct MecanoChar {
    pub c : char,
    pub state : CharState,
}

pub struct MecanoWord {
    word : Vec<MecanoChar>,
    excess : String,
    select_char_idx : usize,
    n_excess : usize,
    state : WordState,
}


pub enum WordState {
    Correct,
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
        return MecanoWord {
            word,
            excess : String::default(),
            select_char_idx : 0,
            n_excess : 0,
            state : WordState::Unreached,
        }
    }

    pub fn type_char(&mut self, c : char) {
        if self.select_char_idx < self.word.len() {
            if self.word[self.select_char_idx].c == c {
                self.word[self.select_char_idx].state = CharState::Correct;
            } else {
                self.word[self.select_char_idx].state = CharState::Wrong;
            }
            self.select_char_idx += 1;
            if self.select_char_idx < self.word.len() {
                self.word[self.select_char_idx].state = CharState::Selected;
            }
        } else {
            self.excess.push(c);
            self.n_excess += 1;
        }
    }

    pub fn delete(&mut self) {
        if self.n_excess > 0 {
            self.excess.pop();
            self.n_excess -= 1;
        } else if self.select_char_idx > 0 {
            self.word
                .get_mut(self.select_char_idx)
                .map(|c| c.state = CharState::Unreached);
            self.select_char_idx -= 1;
            self.word[self.select_char_idx].state = CharState::Selected;
        }
    }

    pub fn total_chars(&self) -> usize {
        return self.word.iter().count() + self.n_excess;
    }

    pub fn select(&mut self) {
        self.state = WordState::Selected;
        self.word[0].state = CharState::Selected;
    }

    pub fn get_punct() {
        todo!();
    }

    pub fn iter(&self) -> Iter<MecanoChar> {
        return self.word.iter();
    }

    pub fn excess(&self) -> &str {
        return &self.excess;
    }
}
