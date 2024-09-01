use constcat::concat;
use crate::NAME;

pub mod flags;
pub mod options;
mod file_health;

const MECANO_PATH : &str = "/usr/share/";
const USER_PATH : &str = "~/.config/";

const CONFIG_FILE : &str = "config.toml";
const MECANO_CONFIG_PATH : &str = concat!(MECANO_PATH, NAME, "/", CONFIG_FILE);
const USER_CONFIG_PATH : &str = concat!(USER_PATH, NAME, "/", CONFIG_FILE);

const DICT_FOLDER : &str = "dictionaries";
const MECANO_DICTS_PATH : &str = concat!(MECANO_PATH, NAME, "/", DICT_FOLDER);
const USER_DICTS_PATH : &str = concat!(USER_PATH, NAME, "/", DICT_FOLDER);
