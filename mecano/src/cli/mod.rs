use std::path::PathBuf;

pub mod flags;
pub mod options;

pub fn dictionaries_path() -> PathBuf {
    let resources_path = root_config_path().join("dictionaries");
    return resources_path;
}

pub fn config_file_path() -> PathBuf {
    let resources_path = root_config_path().join("config.toml");
    return resources_path;
}

fn root_config_path() -> PathBuf {
    let root_config_path = dirs::config_dir()
        .expect("config directory not found")
        .join(crate::NAME);
    return root_config_path;
}

