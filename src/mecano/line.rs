use std::fmt::Display;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crate::config::ConfigLine;

pub struct MecanoLine {
    words : Vec<String>,
    words_state : Vec<WordState>,
    typing_idx : usize,
    config : ConfigLine,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WordState {
    Correct,
    Wrong,
    Selected,
    TypingWrong,
    Unreached,
}

impl MecanoLine {

    pub fn new(words_vec : Vec<String>, config_line : ConfigLine) -> MecanoLine {
        let words_state : Vec<WordState> = vec![WordState::Unreached; words_vec.len()];
        let words : Vec<String> = words_vec;
        let config = config_line;

        return MecanoLine {
            words,
            words_state,
            typing_idx : 0,
            config,
        }
    }

    pub fn typed(&mut self, typed_word : &str) {
        self.words_state[self.typing_idx] = 
            if self.words[self.typing_idx]
                .chars()
                .zip(typed_word.chars())
                .all(|(a, b)|
                    a == b) &&
                self.words[self.typing_idx].len() >= 
                typed_word.len() {
                WordState::Selected
            } else {
                WordState::TypingWrong
            };
        }

    pub fn next_word(&mut self, typed_word : &str) -> Result<WordState, WordState> {

        if self.typing_idx >= self.words.len() - 1{
            if self.words[self.typing_idx] == typed_word {
                return Err(WordState::Correct);
            } else {
                return Err(WordState::Wrong);
            };
        }

        self.words_state[self.typing_idx] = 
            if self.words[self.typing_idx] == typed_word {
                WordState::Correct
            } else {
                WordState::Wrong
            };

        self.typing_idx += 1;
        self.words_state[self.typing_idx] = WordState::Selected;

        return Ok(self.words_state[self.typing_idx - 1]);
    }

    pub fn select(&mut self) {
        self.words_state[0] = WordState::Selected;
    }
}

impl Display for MecanoLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (word, state) in std::iter::zip(&self.words, &self.words_state) {
            match state {
                WordState::Correct => write!(f, "{}{}",
                    SetForegroundColor(self.config.get_fg_correct()),
                    SetBackgroundColor(Color::Reset))?,
                WordState::Wrong => write!(f, "{}{}",
                    SetForegroundColor(self.config.get_fg_wrong()),
                    SetBackgroundColor(Color::Reset))?,
                WordState::Selected => write!(f, "{}{}",
                    SetForegroundColor(Color::Reset),
                    SetBackgroundColor(self.config.get_bg_selected()))?,
                WordState::TypingWrong => write!(f, "{}{}", 
                    SetForegroundColor(Color::Reset),
                    SetBackgroundColor(self.config.get_bg_wrong()))?,
                WordState::Unreached => write!(f, "{}{}", 
                    SetForegroundColor(Color::Reset),
                    SetBackgroundColor(Color::Reset))?,
            }
            write!(f, "{}", word)?;
            write!(f, "{}{}",
                SetForegroundColor(Color::Reset),
                SetBackgroundColor(Color::Reset),
            )?;
            write!(f, " ")?;
        }
        return std::fmt::Result::Ok(());
    }
}


#[cfg(test)]
mod test {
    use super::MecanoLine;
    use super::ConfigLine;
    use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};

    fn build() -> MecanoLine {
        let words_vec : Vec<String> = vec!["Hola".to_string(), "que".to_string(), "tal".to_string(), "estas".to_string()];
        return MecanoLine::new(words_vec, ConfigLine::_default());
    }

    #[test]
    fn build_test() {
        let mecano_line = build();

        assert_eq!(mecano_line.words.len(), 4);
        assert_eq!(mecano_line.words[2], "tal");
    }

    #[test]
    fn colors_display_test() {
        let mut mecano_line = build();
        let _ = mecano_line.next_word("Hola");
        let _ = mecano_line.next_word("Hola");
        let _ = mecano_line.typed("tal");

        let expected = format!("{}{}Hola{}{} {}{}que{}{} {}{}tal{}{} {}{}estas{}{} ", 
            SetForegroundColor(mecano_line.config.get_fg_correct()),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),

            SetForegroundColor(mecano_line.config.get_fg_wrong()),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),

            SetForegroundColor(Color::Reset),
            SetBackgroundColor(mecano_line.config.get_bg_selected()),
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),

            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset),
            SetBackgroundColor(Color::Reset),
        );
        println!("{}", expected); 
        println!("{}", mecano_line); 
        assert_eq!(expected, format!("{mecano_line}"));
    }
}
