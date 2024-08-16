use crossterm::style::Color;
use serde::Deserialize;
use std::io;
use std::time::Duration;

use crate::Count;

use super::find_path_to_file;

use super::TermUnit;

const MODE : &'static str = "file";
const MAX_TIME_MODE : &'static str = "dictionary";
const FILE : &'static str = "100_spanish";
const WIDTH : TermUnit = 80;
const MAX_TIME : Count = 60;
const LINES_TO_SHOW : TermUnit = 2;
const FPS : u16 = 120;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub struct ConfigTextBox {
    pub selected : Color,
    pub wrong : Color,
    pub right : Color,
}

impl ConfigTextBox {
    pub fn default() -> ConfigTextBox {
        return ConfigTextBox {
            selected : Color::Rgb{r : 128, g : 128, b : 128},
            wrong : Color::Rgb{r : 255, g : 128, b : 128},
            right : Color::Rgb{r : 64, g : 255, b : 64},
        };
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    width : Option<TermUnit>,
    max_time : Option<u64>,
    lenght : Option<TermUnit>,
    config_text_box : Option<ConfigTextBox>,
    mode : Option<String>,
    file : Option<String>,
    fps : Option<u16>,
}

impl Config {
    pub fn default() -> Config {
        Config { 
            width : Some(WIDTH),
            max_time : Some(MAX_TIME),
            config_text_box : Some(ConfigTextBox::default()),
            lenght : Some(LINES_TO_SHOW),
            mode : Some(MODE.to_string()),
            file : Some(find_path_to_file(FILE).expect("file not found")),
            fps : Some(FPS),
        }
    }

    pub fn max_time() -> Config {
        Config { 
            width : Some(WIDTH),
            max_time : Some(u64::MAX),
            config_text_box : Some(ConfigTextBox::default()),
            lenght : Some(LINES_TO_SHOW),
            mode : Some(MAX_TIME_MODE.to_string()),
            file : Some(find_path_to_file(FILE).expect("file not found")),
            fps : Some(FPS),
        }
    }

    pub fn from_path(path : &str) -> io::Result<Config> {
        let config = std::fs::read_to_string(path)?;
        return Config::from_str(&config);
    }

    // REFACTOR
    pub fn from_str(str : &str) -> io::Result<Config> {
        let config = toml::from_str::<Config>(&str);
        if let Ok(mut config) = config {
            // Checks file ok
            if let Some(file) = config.file {
                if let Ok(path_to_file) = find_path_to_file(&file) {
                    config.file = Some(path_to_file);
                } else {
                    let error_str = format!("invalid file {file}");
                    println!("{error_str}");
                    return Err(io::Error::new( io::ErrorKind::NotFound, error_str));
                }
            }
            // Here should be compared wheter it is a valid mode or not

            // Checks lines_to_show ok
            if let Some(lines_to_show) = config.lenght {
                if lines_to_show < 1 { config.set_lines_to_show(1) }
            }

            config.set_none_to_default();
            return Ok(config);
        } else {
            let e = config.unwrap_err();
            let error_str = e.message();
            println!("{error_str}");
            return Err(io::Error::new( io::ErrorKind::InvalidData, error_str));
        }
    }

    fn set_none_to_default(&mut self) {
        let default = Config::default();
        if let None = self.mode {
            self.set_mode(default.get_mode().as_str());
        }
        if let None = self.file {
            self.set_file(default.get_file().as_str());
        }
        if let None = self.width {
            self.set_width(default.get_width());
        }
        if let None = self.lenght {
            self.set_lines_to_show(default.get_lenght());
        }
        if let None = self.max_time {
            self.set_max_time(default.get_max_time().as_secs());
        }
        if let None = self.config_text_box {
            self.set_config_text_box(default.get_config_text_box());
        }
        if let None = self.fps {
            self.set_fps(default.get_fps());
        }
    }

    pub fn get_mode(&self) -> String { 
        let mode = self.mode.clone().unwrap();
        return mode;
    }
    pub fn set_mode(&mut self, m : &str) { self.mode = Some(m.to_string()) }

    pub fn get_file(&self) -> String { 
        let file = self.file.clone().unwrap();
        return file;
    }
    pub fn set_file(&mut self, f : &str) { self.file = Some(f.to_string()) }

    pub fn get_width(&self) -> TermUnit { self.width.unwrap() }
    pub fn set_width(&mut self, w : TermUnit) { self.width = Some(w)}

    pub fn get_max_time(&self) -> Duration { Duration::from_secs(self.max_time.unwrap()) }
    pub fn set_max_time(&mut self, m : Count) { self.max_time = Some(m) }

    pub fn get_lenght(&self) -> TermUnit { self.lenght.unwrap() }
    pub fn set_lines_to_show(&mut self, l : TermUnit) { 
        self.lenght = Some(l)
    }

    pub fn get_config_text_box(&self) -> ConfigTextBox { self.config_text_box.unwrap() }
    pub fn set_config_text_box(&mut self, c : ConfigTextBox) { 
        self.config_text_box = Some(c)
    }

    pub fn get_fps(&self) -> u16 { self.fps.unwrap() }
    pub fn set_fps(&mut self, f : u16) { self.fps = Some(f) }
}

#[cfg(test)]

mod test {
    use std::time::Duration;

    use crate::{config::{self, ConfigTextBox}, find_path_to_file};

    use super::Config;


    #[test]
    fn empty_config() {
        let config = Config::from_str("").unwrap();

        assert!(config.get_mode() == config::MODE);
        assert!(config.get_file() == find_path_to_file(config::FILE).unwrap());
        assert!(config.get_width() == config::WIDTH);
        assert!(config.get_max_time() == Duration::from_secs(config::MAX_TIME));
        assert!(config.get_lenght() == config::LINES_TO_SHOW);
        assert!(config.get_config_text_box() == ConfigTextBox::default());
        assert!(config.get_fps() == config::FPS);

    }
}
