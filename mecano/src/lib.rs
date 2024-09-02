pub const NAME : &'static str = "mecano";
pub const VERSION : &'static str = "0.2.0";

use std::io;
use std::io::{Error, ErrorKind};
use std::env;
use std::path::Path;

pub mod engine;
pub mod config;
pub mod mode;
pub mod cli;
pub mod textbox;
pub mod punctuation;

pub type Idx = usize;
pub type TermUnit = u16;
pub type Count = u64;

pub fn path_to_file(input : &str) -> io::Result<String> {
    let mut file = input.to_string();
    if input.chars().next().unwrap_or('\0') == '~' {
        file = env::var("HOME").unwrap() + &file[1..file.chars().count() - 1];
    }
    let home_config = env::var("HOME").unwrap() + "/.config/mecano/";
    let paths_to_search = vec![
        file.clone(),
        home_config.clone() + &file,
        home_config + "dictionaries/" + &file,
        "/usr/share/mecano/".to_string() + &file,
        "/usr/share/mecano/dictionaries/".to_string() + &file,
    ];


    for path in paths_to_search {
        if let Ok(()) = healthy_file(&path) {
            return Ok(path);
        }
    }
    return Err(Error::new(ErrorKind::NotFound, "file not found"));
}

pub fn healthy_file(path : &str) -> io::Result<()> {
    let file_result = std::fs::File::open(Path::new(&path));
    if let Ok(_) = file_result {
        return Ok(());
    } else {
        return Err(Error::new(ErrorKind::NotFound,
            format!("{path}: file not found")));
    }
}
