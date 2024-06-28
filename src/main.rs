use std::{env, fs};
use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::{io::{self, Error, ErrorKind}, time::Duration};
use config::Config;
use crossterm::event::{poll, read, Event, KeyCode};

use mecano::Mecano;

use crate::help::Help;

mod config;
mod mecano;
mod modes;
mod help;

fn main() -> io::Result<()> {

    let config_path = 
    env::var("HOME").expect("home directory not found") + "/.config/mecano";
    copy_dir_all("/usr/share/mecano", config_path.clone())?;
    let log_path = config_path + "/mecano.log";
    let log_file = File::create(log_path)?;
    let log_fd = log_file.as_raw_fd();
    unsafe {
        libc::dup2(log_fd, libc::STDERR_FILENO);
    }

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter().skip(1);

    let mut config : Config;
    if let Ok(c) = Config::from_path(&find_path_to_file("mecano.toml")?) {
        config = c;
    } else {
        println!("invalid configuration in `mecano.toml`");
        return Err(Error::new(ErrorKind::InvalidData, "invalid configuration"));
    }

    while let Some(item) = args_iter.next() {
        match item.as_str() {
            "-d" | "--dictionary" => {
                let file : &str;
                if let Some(f) = args_iter.next() {
                    file = f;
                } else {
                    println!("missing argument for --file");
                    return Ok(());
                };

                if let Ok(path) = find_path_to_file(file) {
                    config.file = path;
                    config.mode = "dictionary".to_string();
                } else {
                    println!("file not found: {}", file);
                    return Ok(());
                }
            }

            "-f" | "--file" => {
                let file : &str;
                if let Some(f) = args_iter.next() {
                    file = f;
                } else {
                    println!("missing argument for --file");
                    return Ok(());
                };

                if let Ok(path) = find_path_to_file(file) {
                    config.file = path;
                    config.mode = "file".to_string();
                } else {
                    println!("file not found: {}", file);
                    return Ok(());
                }
            }

            "-t" | "--time" => {
                config.max_time = args_iter
                    .next()
                    .unwrap_or(&"0".to_string())
                    .parse::<u64>()
                    .unwrap_or(0);

                if config.max_time < 2 {
                    println!("invalid or missing time");
                    return Ok(());
                }
            }

            "-h" | "--help" => {
                println!("{}", Help::help_text());
                return Ok(());
            }

            "-v" | "--version" => {
                println!("mecano 0.1.0");
                return Ok(());
            }

            "-l" | "--list-dictionaries" => {
                let path_to_dictionaries;
                if let Ok(p) = find_path_to_file("dictionaries") {
                    path_to_dictionaries = p;
                } else {
                    println!("path to dictionaries not found. check `~/.config/mecano/dictionaries` or `/usr/share/mecano/dictionaries`");
                    return Err(Error::new(
                        ErrorKind::NotFound, "dictionaries not found"));
                }
                for file in fs::read_dir(path_to_dictionaries)? {
                    println!("{}", file?
                        .file_name()
                        .to_str()
                        .unwrap());
                }
                return Ok(());
            }

            _ => {
                println!("invalid argument: {}", item);
                return Ok(());
            },
        }
    }

    let mut mecano = Mecano::start(config)?;

    while !mecano.is_ended() {
        if let Ok(true) = poll(Duration::from_secs(0)) { break; }
        if let Ok(event) = read() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Esc => {
                        break;
                    },
                    _ => {
                        mecano.type_key_event(key_event)?
                    }
                }
            }
        }
    }

    return Ok(());
}

fn find_path_to_file(input : &str) -> io::Result<String> {
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
        eprintln!("{}", path);
        if let Ok(()) = healthy_file(&path) {
            eprintln!("Chosen: {}", path);
            return Ok(path);
        }
    }
    return Err(Error::new(ErrorKind::NotFound, "file not found"));
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    return Ok(());
}

fn healthy_file(path : &str) -> io::Result<()> {
    let file_result = std::fs::File::open(Path::new(&path));
    if let Ok(_) = file_result {
        return Ok(());
    } else {
        return Err(Error::new(ErrorKind::NotFound,
            format!("{path}: file not found")));
    }
}
