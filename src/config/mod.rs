use crossterm::style::Color;

use crate::dictionary::Dictionary;

use std::time::Duration;

pub enum Mode {
    Dictionary(Dictionary),
    File,
}

#[derive(Clone, Copy)]
pub struct ConfigLine {
    bg_selected : Color,
    bg_wrong : Color,
    fg_wrong : Color,
    fg_correct : Color,
}

impl ConfigLine {
    pub fn default() -> ConfigLine {
        return ConfigLine {
            bg_selected : Color::Rgb{r : 100, g : 100, b : 100},
            bg_wrong : Color::Rgb{r : 200, g : 128, b : 128},
            fg_correct : Color::Rgb{r : 64, g : 200, b : 64},
            fg_wrong : Color::Rgb{r : 200, g : 64, b : 64},
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

pub struct Config {
    pub mode : Mode,
    pub debug : bool,
    pub max_time : std::time::Duration,
    pub lines_to_show : usize,
    pub config_line : ConfigLine,
}

impl Config {
    pub fn default() -> Config {
        Config { 
            mode : Mode::File,
            debug : true,
            max_time : Duration::from_secs(60),
            config_line : ConfigLine::default(),
            lines_to_show : 2,
        }
    }

    pub fn get_max_time(&self) -> Duration {
        return self.max_time;
    }
}
