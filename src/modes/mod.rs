use self::{dictionary::MecanoDictionary, file::MecanoFile};

pub mod dictionary;
pub mod file;

pub trait MecanoMode {
    fn yield_words(&mut self) -> Vec<String>;
}

pub enum Mode {
    File(MecanoFile),
    Dictionary(MecanoDictionary),
}

impl Mode {
    pub fn yield_words(&mut self) -> Vec<String> {

        return match self {
            Mode::File(mecano_file) => mecano_file.yield_words(),
            Mode::Dictionary(mecano_dictionary) => mecano_dictionary.yield_words(),
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
