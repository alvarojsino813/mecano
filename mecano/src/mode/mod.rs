pub mod dictionary;
pub mod file;
pub mod mode;

pub use file::SourceFile;
pub use dictionary::SourceDictionary;

use crate::{config::Config, TermUnit};

pub trait WordSource {
    fn yield_word(&mut self) -> &str;

    // REFACTOR
    fn yield_words(&mut self) -> Vec<String> {
        let mut words_yielded : Vec<String> = Vec::new();

        let mut width = 0;

        while width <= 80 {
            let rand_word = self.yield_word();
            width += rand_word.chars().count() as TermUnit + 1;
            words_yielded.push(rand_word.to_string());
        }

        return words_yielded;
    }

    fn name(&self) -> String;

    fn from_config(config : &Config) -> Self where Self : Sized;
}
