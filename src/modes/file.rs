use super::MecanoMode;

pub struct MecanoFile {
    file_words: Vec<String>,
    nth_word: usize,
    max_width : u16,
}

impl MecanoFile {
    pub fn new(path_to_file : &str, max_width : u16) -> MecanoFile {

        let mut file_words : Vec<String> = Vec::new();
        let contents; 
        if let Ok(c) = std::fs::read_to_string(path_to_file) {
            contents = c;
        } else {
            println!("file not found or corrupted");
            panic!("file not found or corrupted");
        }

        for word in contents.split_whitespace() {
            if word.chars().count() < max_width as usize {
                file_words.push(word.to_string());
            }
        }

        if file_words.is_empty() {
            println!("`width` too low");
            panic!("`width too low");
        }

        return MecanoFile {
            file_words,
            nth_word : 0,
            max_width,
        };
    }
}

impl MecanoMode for MecanoFile {
    fn yield_words(&mut self) -> Vec<String> {
        let mut words_yielded : Vec<String> = Vec::new();
        let mut width = 0;

        while 
        width + self.file_words[self.nth_word].chars().count() 
        < 
        self.max_width as usize {
            words_yielded.push(self.file_words[self.nth_word].clone());
            width += self.file_words[self.nth_word].chars().count() + 1;
            self.nth_word = (self.nth_word + 1) % self.file_words.len();
        }

        return words_yielded;
    }
}
