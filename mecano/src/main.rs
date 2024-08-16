use std::time::Duration;
use std::{env, fs};
use std::io::{self, Error, ErrorKind};

use mecano::config::Config;

use mecano::engine::Mecano;
use mecano::{find_path_to_file, healthy_file};

use mecano::cli::response_text::help_text;

fn main() -> io::Result<()> {

    let config_path = 
    env::var("HOME").expect("home directory not found") + "/.config/mecano";
    if let Err(_) = healthy_file(&(config_path.clone() + "/mecano.toml")) {
        fs::copy("/usr/share/mecano/mecano.toml", config_path + "/mecano.toml")?;
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
                    config.set_file(&path);
                    config.set_mode("dictionary");
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
                    config.set_file(&path);
                    config.set_mode("file");
                } else {
                    println!("file not found: {}", file);
                    return Ok(());
                }
            }

            "-t" | "--time" => {
                let max_time = args_iter
                    .next()
                    .unwrap_or(&"0".to_string())
                    .parse::<u64>()
                    .unwrap_or(0);

                config.set_max_time(max_time);

                if config.get_max_time() < Duration::from_secs(1) {
                    println!("invalid or missing time");
                    return Ok(());
                }
            }

            "-h" | "--help" => {
                println!("{}", help_text());
                return Ok(());
            }

            "-v" | "--version" => {
                println!("mecano v0.1.2");
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

    Mecano::play(config)?;

    return Ok(());
}

