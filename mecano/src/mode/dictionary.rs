use crate::{mode::WordSource, Idx};

pub struct SourceDictionary {
    possible_words: Vec<String>,
}

impl SourceDictionary {
    pub fn new(path_to_dictionary : &str) -> SourceDictionary {
        let mut possible_words : Vec<String> = Vec::new();
        let contents = std::fs::read_to_string(path_to_dictionary).unwrap();

        for word in contents.split_whitespace() {
            possible_words.push(word.to_string());
        }

        if possible_words.is_empty() {
            println!("`width` too low");
            panic!("`width too low");
        }

        return SourceDictionary {
            possible_words,
        };
    }
}

impl WordSource for SourceDictionary {
    fn yield_word(&mut self) -> &str {
        let word = self.possible_words
            [random(self.possible_words.len() - 1)].as_str();

        return word;
    }

    fn name(&self) -> String { String::from("dictionary") }

    fn from_config(config : &crate::config::Config) -> Self {
        return Self::new(&config.get_file());
    }
}

fn random(top : Idx) -> Idx {
    return (rand::random::<f32>() * top as f32) as Idx;
}

#[cfg(test)]
mod test {

    use crate::find_path_to_file;

    use super::SourceDictionary;
    use super::WordSource;

    #[test]
    fn printing_words() {

        let dict = SourceDictionary::new(
            &find_path_to_file("100_spanish").unwrap());

        assert_eq!(dict.possible_words.len(), 100);
    }

    #[test]
    fn true_random() {

        let mut dict = SourceDictionary::new(
            &find_path_to_file("100_spanish").unwrap());

        let left_line = dict.yield_words();
        let right_line = dict.yield_words();

        assert!(!left_line.iter().eq(right_line.iter()));
    }
}
