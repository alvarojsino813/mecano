pub mod dictionary;
pub mod file;

pub use file::SourceFile;
pub use dictionary::SourceDictionary;

use crate::{config::Config, TermUnit};

pub const ALL_MODES : &'static [&'static str] = &["dictionary", "file"];

pub fn all_modes_str() -> String {
    let mut all_modes_str = String::new();
    for mode in ALL_MODES.iter().take(ALL_MODES.len() - 1) {
        all_modes_str.push_str(&format!("\"{mode}\""));
        all_modes_str.push_str(", ");
    }
    let last_mode = ALL_MODES.last();
    if let Some(last_mode) = last_mode {
        all_modes_str.push_str(&format!("\"{last_mode}\""));
    }
    return all_modes_str;
}

pub trait WordSource {
    fn yield_word(&mut self) -> &str;

    fn yield_words(&mut self) -> Vec<String> {
        let mut words_yielded : Vec<String> = Vec::new();

        let mut width = 0;

        while width <= Config::default().get_width() {
            let rand_word = self.yield_word();
            width += rand_word.chars().count() as TermUnit + 1;
            words_yielded.push(rand_word.to_string());
        }

        return words_yielded;
    }

    fn name(&self) -> String;

    fn from_config(config : &Config) -> Self where Self : Sized;
}
