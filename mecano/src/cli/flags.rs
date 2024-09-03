use std::{collections::HashSet, fs::read_dir, io, path::PathBuf};

use crate::{mode::ALL_MODES, NAME, VERSION};

use super::dictionaries_path;

pub fn flags(args : &Vec<String>) -> String {
    let mut flags : String = String::new();
    let mut args_iter = args.iter();
    while let Some(item) = args_iter.next() {
        match item.as_str() {
            "-h" | "--help" => {
                flags = help_flag().to_string();
            }

            "--list-dictionaries" => {
                flags = list_dicts_flag();
            }

            "--list-modes" => {
                flags = list_modes_flag();
            }

            "-v" | "--version" => {
                flags = version_flag();
            }

            _ => ()
        }
    }

    return flags;
}

fn version_flag() -> String {
    return format!("{NAME} v{VERSION}");
}

fn list_modes_flag() -> String {
    let mut modes_msg = String::new();
    let first_mode = ALL_MODES.first().unwrap_or(&&"");
    modes_msg.push_str(&format!("{first_mode}"));
    for mode_name in ALL_MODES.iter().skip(1) {
        modes_msg.push_str(&format!("\n{mode_name}"));
    }
    return modes_msg;
}

fn list_dicts_flag() -> String {

    let mut all_dicts : HashSet<String> = HashSet::new();

    all_dicts.extend(file_names_in_dir(&dictionaries_path()).unwrap_or_default());

    let mut all_dicts : Vec<String> = all_dicts.into_iter().collect();

    all_dicts.sort();

    let mut list_dicts_msg = String::new();

    let empty = String::new();
    let first_dict = all_dicts.first().unwrap_or(&empty);
    list_dicts_msg.push_str(&first_dict);
    for dict_name in all_dicts.iter().skip(1) {
        list_dicts_msg.push_str(&format!("\n{dict_name}"));
    }

    return list_dicts_msg;
}

fn file_names_in_dir(path : &PathBuf) -> io::Result<HashSet<String>> {
    let mut set : HashSet<String> = HashSet::new();
    for file in read_dir(path)? {
        let file = file?;
        if file.file_type()?.is_file() {
            let file_name = file.file_name();
            let file_name = file_name.to_str().unwrap_or("\0");
            set.insert(file_name.to_string());
        } else if file.file_type()?.is_dir() {
            // TO DO : 
            // let inner_set = file_names_in_dir(file.file_name().to_str().unwrap_or("\0"));
            // set.extend(inner_set.unwrap_or_default());
        }
    }
    return Ok(set);
}

fn help_flag() -> &'static str {
"Mecano, a typing train

Usage: mecano [OPTIONS] [FLAGS]

OPTIONS:
-f, --file <FILE>           Plays using the chosen file or dictionary
-m, --mode <MODE>           Plays the chosen mode
-r, --rate <RATE>           Plays with the chosen rate. This affects time measures accuracy. The higher the better.
-t, --time <SECS>           Choose the game time in seconds

FLAGS:
-h, --help                  Print help
-v, --version               Print version 
    --list-dictionaries     List all dicitonaries. You can add more at ~/.config/mecano/dictionaries
    --list-modes            List all available modes"
}


