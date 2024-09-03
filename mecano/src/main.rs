use std::env;

use mecano::cli::flags::flags;
use mecano::cli::options::config_with_args;
use mecano::engine::Mecano;

fn main() {

    let args: Vec<String> = env::args().collect();

    let flags_msg = flags(&args);
    if !flags_msg.is_empty() {
        println!("{flags_msg}");
        return;
    }

    let config = config_with_args(&args);
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

