use std::{env, fs::File, io::{self, Error, ErrorKind}};

use super::*;

const CONFIG_PATHS : &'static [&'static str] = &[
    USER_CONFIG_PATH, 
    MECANO_CONFIG_PATH,
];

const DICTS_PATHS : &'static [&'static str] = &[
    USER_DICTS_PATH, 
    MECANO_DICTS_PATH,
];


pub fn find_config_path(file_name : &str) -> io::Result<String> {
    for path in CONFIG_PATHS {
        if healthy_file(&extend_home_path(&path)) {
            return Ok(path.to_string());
        }
    }
    return Err(Error::new(ErrorKind::NotFound,
        format!("{file_name} not found")));
}

pub fn find_dict_path(file_name : &str) -> io::Result<String> {
    for path in DICTS_PATHS {
        if healthy_file(&extend_home_path(&path)) {
            return Ok(path.to_string());
        }
    }
    return Err(Error::new(ErrorKind::NotFound,
        format!("{file_name} not found")));
}

fn healthy_file(file_name : &str) -> bool {
    let file = File::open(file_name);
    return if let Ok(_) = file {
        true
    } else {
        false
    };
}

fn extend_home_path(path : &str) -> String {
    let mut extended_path = path.to_string();
    if path.chars().next().unwrap_or('\0') == '~' {
        extended_path = env::var("HOME").unwrap() + 
            &extended_path[1..extended_path.chars().count() - 1];
    }
    return extended_path;
}
