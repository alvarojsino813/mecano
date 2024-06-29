use super::MecanoMode;

pub struct MecanoDictionary {
    possible_words: Vec<String>,
    max_width : u16,
}

impl MecanoDictionary {
    pub fn new(path_to_dictionary : &str, max_width : u16) -> MecanoDictionary {
        let mut possible_words : Vec<String> = Vec::new();
        let contents = std::fs::read_to_string(path_to_dictionary).unwrap();

        for word in contents.split_whitespace() {
            if word.chars().count() < max_width as usize {
                possible_words.push(word.to_string());
            }
        }

        if possible_words.is_empty() {
            println!("`width` too low");
            panic!("`width too low");
        }

        return MecanoDictionary {
            possible_words,
            max_width,
        };
    }
}

impl MecanoMode for MecanoDictionary {
    fn yield_words(&mut self) -> Vec<String> {
        let mut words_yielded : Vec<String> = Vec::new();

        let mut width = 0;

        while width <= self.max_width {
            let rand_word = self.possible_words
                [random(self.possible_words.len() - 1)].as_str();
            width += rand_word.chars().count() as u16 + 1;
            words_yielded.push(rand_word.to_string());
        }

        words_yielded.pop();

        return words_yielded;
    }
}

fn random(top : usize) -> usize {
    return (rand::random::<f32>() * top as f32) as usize;
}

#[cfg(test)]
mod test {

    use crate::find_path_to_file;

    use super::MecanoDictionary;
    use super::MecanoMode;

    #[test]
    fn printing_words() {

        let dict = MecanoDictionary::new(
            &find_path_to_file("100_spanish").unwrap(), 80);

        assert_eq!(dict.possible_words.len(), 100);
    }

    #[test]

    fn true_randome() {

        let mut dict = MecanoDictionary::new(
            &find_path_to_file("100_spanish").unwrap(), 80);

        let left_line = dict.yield_words();
        let right_line = dict.yield_words();

        assert!(!left_line.iter().eq(right_line.iter()));
    }
}
