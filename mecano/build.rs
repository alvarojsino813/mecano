use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> std::io::Result<()> {

    println!("cargo::rerun-if-changed=build.rs");

    let mut resources = env::current_dir()
        .expect("Source directory not found")
        .join("resources");

    if !resources.exists() {
    resources = env::current_dir()
        .expect("Source directory not found")
        .join("mecano")
        .join("resources");
    }

    if !resources.exists() {
        let error_msg = format!("Resources not found. Build from the root of the project");
        return Err(io::Error::new(io::ErrorKind::NotFound, error_msg));
    }

    let config_path = dirs::config_dir()
        .expect("Config directory not found")
        .join("mecano");

    copy_dir_to_dir(resources, config_path)?;

    return Ok(());
}

fn copy_dir_to_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> std::io::Result<()> {

    let from_path = from.as_ref();
    let from_display = from_path.display();
    let to_path = to.as_ref();

    fs::create_dir_all(&to)?;

    if !from_path.try_exists()? {
        let error_msg = format!("{from_display} doesn't exist");
        return Err(io::Error::new(io::ErrorKind::NotFound, error_msg));
    }

    if !from_path.is_dir() {
        let error_msg = format!("{from_display} is not a directory");
        return Err(io::Error::new(io::ErrorKind::Other, error_msg));
    }

    for entry in from_path.read_dir()? {
        if let Ok(entry) = entry {
            let file_path = &entry.path();
            let file_name = &entry.file_name();
            if file_path.is_dir() { 
                copy_dir_to_dir(file_path, to_path.join(file_name))?; 
            } else if !to_path.join(file_name).exists() { 
                fs::copy(file_path, to_path.join(file_name))?; 
            }
        }
    }
    return Ok(());
}
