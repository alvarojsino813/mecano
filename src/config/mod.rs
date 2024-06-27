use crossterm::style::Color;
use serde::Deserialize;
use std::io;
use std::time::Duration;

use crate::find_path_to_file;
use crate::modes::Mode;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ConfigLine {
    bg_selected : Color,
    bg_wrong : Color,
    fg_wrong : Color,
    fg_correct : Color,
}

impl ConfigLine {
    pub fn _default() -> ConfigLine {
        return ConfigLine {
            bg_selected : Color::Rgb{r : 128, g : 128, b : 128},
            bg_wrong : Color::Rgb{r : 255, g : 128, b : 128},
            fg_correct : Color::Rgb{r : 64, g : 255, b : 64},
            fg_wrong : Color::Rgb{r : 255, g : 64, b : 64},
        };
    }

    pub fn get_bg_selected(&self) -> Color {
        return self.bg_selected;
    }

    pub fn get_bg_wrong(&self) -> Color {
        return self.bg_wrong;
    }

    pub fn get_fg_wrong(&self) -> Color {
        return self.fg_wrong;
    }

    pub fn get_fg_correct(&self) -> Color {
        return self.fg_correct;
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub width : u16,
    pub max_time : u64,
    pub lines_to_show : u16,
    pub config_line : ConfigLine,
    pub mode : String,
    pub file : String,
}

impl Config {
    pub fn _default() -> Config {
        Config { 
            width : 80,
            max_time : 60,
            config_line : ConfigLine::_default(),
            lines_to_show : 2,
            mode : "dictionary".to_string(),
            file : find_path_to_file("100_spanish").expect("file not found"),
        }
    }

    pub fn from_path(path : &str) -> io::Result<Config> {
        let config = std::fs::read_to_string(path)?;
        return Config::from_str(&config);
    }

    pub fn from_str(str : &str) -> io::Result<Config> {
        let config = toml::from_str::<Config>(&str);
        if let Ok(mut config) = config {
            if let Ok(path_to_file) = find_path_to_file(&config.file) {
                config.file = path_to_file;
            } else {
                println!("invalid file `{}`", config.file);
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("invalid path: {}", config.file)));
            }
            if config.lines_to_show < 1 { config.lines_to_show = 1 }
            if !Mode::valid_str(&config.mode) {
                println!("invalid mode. expected one among {}", 
                    Mode::all_modes());

                return Err(io::Error::new(
                    io::ErrorKind::InvalidData, "invalid configuration"));
            }
            return Ok(config);
        } else {
            let e = config.unwrap_err();
            println!("{}", e.message());
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, "invalid configuration"));
        }
    }

    pub fn get_max_time(&self) -> Duration {
        return Duration::from_secs(self.max_time);
    }
}
