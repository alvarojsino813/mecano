use std::path::PathBuf;

use crate::{mode::WordSource, Idx};

pub struct SourceFile {
    file_words: Vec<String>,
    nth_word: Idx,
}

impl SourceFile {
    pub fn new(path_to_file : &PathBuf) -> SourceFile {
        let mut file_words : Vec<String> = Vec::new();
        let contents; 
        if let Ok(c) = std::fs::read_to_string(path_to_file) {
            contents = c;
        } else {
            println!("file not found or corrupted");
            panic!("file not found or corrupted");
        }

        for word in contents.split_whitespace() {
            file_words.push(word.to_string());
        }

        return SourceFile {
            file_words,
            nth_word : 0,
        };
    }
}

impl WordSource for SourceFile {
    fn yield_word(&mut self) -> &str {
        let word = &self.file_words[self.nth_word];
        self.nth_word = (self.nth_word + 1) % self.file_words.len();
        return word;
    }

    fn from_config(config : &crate::config::Config) -> Self {
        return Self::new(&config.get_file());
    }

    fn name(&self) -> String { String::from("file") }
}

#[cfg(test)]

mod test {
    use crate::{path_to_file, mode::WordSource};

    use super::SourceFile;


    #[test]
    fn file_deterministic() {
        let path = &path_to_file("100_spanish").unwrap();
        let mut mecano_file = SourceFile::new(path);

        let contents = std::fs::read_to_string(path).unwrap();


        for word in contents.split_whitespace() {
            assert_eq!(word, mecano_file.yield_word());
        }
        
    }
}
