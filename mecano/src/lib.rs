pub const NAME : &'static str = "mecano";
pub const VERSION : &'static str = "0.2.2";

use std::env::current_dir;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use cli::dictionaries_path;

pub mod engine;
pub mod config;
pub mod mode;
pub mod cli;
pub mod textbox;
pub mod punctuation;

pub type Idx = usize;
pub type TermUnit = u16;
pub type Count = u64;

pub fn path_to_file(input : &str) -> io::Result<PathBuf> {
    let path = current_dir().expect("couldn't read current dir").join(input);
    let dict_path = dictionaries_path().join(input);
    let paths_to_search = vec![
        &path,
        &dict_path,
    ];

    for path in paths_to_search {
        if path.exists() {
            let out = path.clone();
            return Ok(out);
        }
    }

    let path = path.display();
    let dict_path = dict_path.display();
    let error_msg = format!("{input} not found at {path} neither at {dict_path}");
    return Err(Error::new(ErrorKind::NotFound, error_msg));
}
