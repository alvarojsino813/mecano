use std::{env, fs};

use mecano::cli::flags::flags;
use mecano::cli::options::config_from_args;

use mecano::engine::Mecano;
use mecano::healthy_file;

fn main() {

    let config_path = 
    env::var("HOME").expect("home directory not found") + "/.config/mecano";
    if let Err(_) = healthy_file(&(config_path.clone() + "/mecano.toml")) {
        let _ = 
        fs::copy("/usr/share/mecano/mecano.toml", config_path + "/mecano.toml");
    }

    let args: Vec<String> = env::args().collect();

    let flags_msg = flags(&args);
    if !flags_msg.is_empty() {
        println!("{flags_msg}");
        return;
    }

    let config = config_from_args(&args);

    if let Err(e) = &config {
        eprintln!("{e}");
        return;
    }

    let config = config.unwrap();
    let result = Mecano::play(config);

    if let Err(e) = result {
        let e_kind = e.kind();
        eprintln!("Error during game : {e_kind}");
    }
}

