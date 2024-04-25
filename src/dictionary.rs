pub struct Dictionary {
    possible_words: Vec<String>
}

impl Dictionary {

    pub fn new(path_to_dictionary : &str) -> Dictionary {

        let mut possible_words : Vec<String> = Vec::new();

        let contents = std::fs::read_to_string(path_to_dictionary).unwrap();

        for word in contents.split_whitespace() {
            possible_words.push(word.to_string());
        }

        let dictionary = Dictionary {
            possible_words
        };

        return dictionary;
    }

    pub fn yield_words(&self, max_width : u16) -> Vec<String> {
        let mut words_yielded : Vec<String> = Vec::new();

        let mut width = 0;

        while width <= max_width {
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

    use super::Dictionary;

    #[test]
    fn printing_words() {

        let dict = Dictionary::new("100_spanish");

        assert!(dict.possible_words.len() == 100);

    }
}
