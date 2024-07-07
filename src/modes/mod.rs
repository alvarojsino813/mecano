use self::{dictionary::MecanoDictionary, file::MecanoFile};

pub mod dictionary;
pub mod file;

pub trait MecanoMode {
    fn yield_word(&mut self) -> &str;

    fn yield_words(&mut self) -> Vec<String> {
        let mut words_yielded : Vec<String> = Vec::new();

        let mut width = 0;

        while width <= 80 {
            let rand_word = self.yield_word();
            width += rand_word.chars().count() as u16 + 1;
            words_yielded.push(rand_word.to_string());
        }

        return words_yielded;
    }
}

pub enum Mode {
    File(MecanoFile),
    Dictionary(MecanoDictionary),
}

impl Mode {
    pub fn yield_word(&mut self) -> &str {

        return match self {
            Mode::File(f) => f.yield_word(),
            Mode::Dictionary(d) => d.yield_word(),
        }
    }

    pub fn new(mode : &str, file : &str, max_width : u16) -> Result<Mode, ()> {

        match mode {
            "file" => return Ok(Mode::File(MecanoFile::new(file, max_width))),
            "dictionary" => return Ok(Mode::Dictionary(MecanoDictionary::new(file, max_width))),
            _ => return Err(()),
        }
    }

    pub fn valid_str(str : &str) -> bool {
        match str {
            "file" | "dictionary" => return true,
            _ => return false,
        }
    }

    pub fn all_modes() -> &'static str {
        return "`dictionary`, `file`";
    }
}
