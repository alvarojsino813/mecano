use crossterm::style::Color;
use serde::Deserialize;
use std::io;
use std::time::Duration;

use crate::Count;

use self::fields::FieldError;
use self::fields::FileField;
use self::fields::ModeField;

use super::path_to_file;

use super::TermUnit;

pub mod fields;

const MODE : &'static str = "file";
const MAX_TIME_MODE : &'static str = "dictionary";
const FILE : &'static str = "100_spanish";
const WIDTH : TermUnit = 80;
const MAX_TIME : Count = 60;
const LINES_TO_SHOW : TermUnit = 2;
const RATE : u16 = 1000;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct Theme {
    pub selected : Option<Color>,
    pub wrong : Option<Color>,
    pub right : Option<Color>,
}

impl Theme {
    pub fn default() -> Theme {
        return Theme {
            selected : Some(Color::Rgb{r : 128, g : 128, b : 128}),
            wrong : Some(Color::Rgb{r : 255, g : 128, b : 128}),
            right : Some(Color::Rgb{r : 64, g : 255, b : 64}),
        };
    }

    pub fn get_selected(&self) -> Color {
        if let Some(c) = self.selected {
            return c;
        } else {
            return Theme::default().selected.unwrap();
        }
    }

    pub fn get_wrong(&self) -> Color {
        if let Some(c) = self.wrong {
            return c;
        } else {
            return Theme::default().wrong.unwrap();
        }
    }

    pub fn get_right(&self) -> Color {
        if let Some(c) = self.right {
            return c;
        } else {
            return Theme::default().right.unwrap();
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Config {
    width : Option<TermUnit>,
    max_time : Option<u64>,
    lenght : Option<TermUnit>,
    theme : Option<Theme>,
    mode : Option<ModeField>,
    file : Option<FileField>,
    rate : Option<u16>,
}

impl Config {
    pub fn empty() -> Config {
        Config { 
            width : None,
            max_time : None,
            theme : None,
            lenght : None,
            mode : None,
            file : None,
            rate : None,
        }
    }

    pub fn default() -> Config {
        Config { 
            width : Some(WIDTH),
            max_time : Some(MAX_TIME),
            theme : Some(Theme::default()),
            lenght : Some(LINES_TO_SHOW),
            mode : Some(ModeField::new(MODE).unwrap()),
            file : Some(FileField::new(FILE).unwrap()),
            rate : Some(RATE),
        }
    }

    pub fn max_time() -> Config {
        Config { 
            width : Some(WIDTH),
            max_time : Some(u64::MAX),
            theme : Some(Theme::default()),
            lenght : Some(LINES_TO_SHOW),
            mode : Some(ModeField::new(MAX_TIME_MODE).unwrap()),
            file : Some(FileField::new(FILE).unwrap()),
            rate : Some(RATE),
        }
    }

    pub fn from_path(path : &str) -> io::Result<Config> {
        let config = std::fs::read_to_string(path)?;
        // TO DO: Should be printed by main
        return Config::from_str(&config);
    }

    fn from_str(string : &str) -> io::Result<Config> {
        // let mut error_msg = String::new();
        let config = toml::from_str::<Config>(&string);
        if let Ok(config) = config {
            return Ok(config);
        } else {
            let e = config.unwrap_err();
            let error_msg = e.message();
            // TO DO: Should be printed by main
            eprintln!("{error_msg}");
            return Err(io::Error::new(io::ErrorKind::InvalidData, error_msg));
        }
    }

    // REFACTOR GET AND SET

    pub fn get_mode(&self) -> String { 
        if let Some(mode) = &self.mode {
            return mode.to_string();
        } else {
            return Config::default().mode.unwrap().to_string();
        }
    }

    pub fn set_mode(&mut self, m : &str) -> Option<FieldError> { 
        let mode = ModeField::new(m);
        if let Ok(mode) = mode {
            self.mode = Some(mode);
            return None;
        } else {
            return Some(FieldError::InvalidMode);
        }
    }

    pub fn get_file(&self) -> String { 
        if let Some(file) = &self.file {
            return file.to_string();
        } 
        return Config::default().file.unwrap().to_string();
    }

    pub fn set_file(&mut self, f : &str) -> Option<FieldError> { 
        let file = FileField::new(f);
        if let Ok(file) = file {
            self.file = Some(file);
            return None;
        } else {
            return Some(FieldError::InvalidFile);
        }
    }

    pub fn get_width(&self) -> TermUnit { 
        if let Some(width) = &self.width {
            return *width;
        } else {
            return Config::default().width.unwrap();
        }
    }

    pub fn set_width(&mut self, w : TermUnit) -> Option<FieldError> { 
        if w < 1 {
            return Some(FieldError::ZeroNotAllowed)
        }
        self.width = Some(w);
        return None;
    }

    pub fn get_max_time(&self) -> Duration { 
        if let Some(max_time) = &self.max_time {
            return Duration::from_secs(*max_time);
        } else {
            let default_time_secs = Config::default().max_time.unwrap();
            return Duration::from_secs(default_time_secs);
        }
    }
    pub fn set_max_time(&mut self, m : Count) -> Option<FieldError> { 
        if m < 1 {
            return Some(FieldError::ZeroNotAllowed)
        }
        self.max_time = Some(m);
        return None;
    }

    pub fn get_lenght(&self) -> TermUnit {
        if let Some(lenght) = &self.lenght {
            return *lenght;
        } else {
            return Config::default().lenght.unwrap();
        }
    }
    pub fn set_lenght(&mut self, l : TermUnit) -> Option<FieldError> {
        if l < 1 {
            return Some(FieldError::ZeroNotAllowed)
        }
        self.lenght = Some(l);
        return None;
    }

    pub fn get_rate(&self) -> u16 {
        if let Some(rate) = &self.rate {
            return *rate;
        } else {
            return Config::default().rate.unwrap();
        }
    }
    pub fn set_rate(&mut self, f : u16) -> Option<FieldError> {
        if f < 1 {
            return Some(FieldError::ZeroNotAllowed)
        }
        self.lenght = Some(f);
        return None;
    }

    // TO DO: Set a nice ConfigTextBox

    pub fn get_config_text_box(&self) -> Theme { 
        if let Some(config_text_box) = self.theme {
            return config_text_box;
        } else {
            return Theme::default();
        }
    }
    pub fn set_config_text_box(&mut self, c : Theme) { 
        self.theme = Some(c)
    }

}

#[cfg(test)]

mod test {
    use std::time::Duration;

    use crate::{config::{self, Theme}, path_to_file};

    use super::Config;


    #[test]
    fn empty_config() {
        let config = Config::from_str("").unwrap();

        assert!(config.get_mode() == config::MODE);
        assert!(config.get_file() == path_to_file(config::FILE).unwrap());
        assert!(config.get_width() == config::WIDTH);
        assert!(config.get_max_time() == Duration::from_secs(config::MAX_TIME));
        assert!(config.get_lenght() == config::LINES_TO_SHOW);
        assert!(config.get_config_text_box() == Theme::default());
        assert!(config.get_rate() == config::RATE);

    }

    #[test]
    fn error_thrown() {

        let _result = Config::from_str("
width = 80
max_time = 60
lines_to_show = 14
file = \"100_spanish\"
mode = 123
rate = -143
[config_text_box]
selected = \"#888888\"
wrong = \"#FF8888\"
right = \"#44FF44\"
        ");

        eprintln!("Holaaa");

    }
}
