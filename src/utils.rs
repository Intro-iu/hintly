use std::fs;
use std::io;
use std::path::PathBuf;
use dirs::config_dir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

pub fn get_config_path() -> Option<PathBuf> {
    let config_dir = config_dir()?;
    let config_path = config_dir.join("hintly");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).expect("Failed to create config directory");
    }
    Some(config_path.join("config.toml"))
}

pub fn is_config_exists() -> bool {
    let config_file = match get_config_path() {
        Some(path) => path,
        None => return false,
    };
    config_file.exists()
}

pub fn read_config() -> io::Result<Config> {
    if !is_config_exists() {
        let config_file = match get_config_path() {
            Some(path) => path,
            None => {
                eprintln!("Failed to get config path.");
                std::process::exit(1);
            }
        };

        eprintln!("Configuration file not found: {}", config_file.display());
        std::process::exit(0);
    } else {
        let config_file = match get_config_path() {
            Some(path) => path,
            None => {
                eprintln!("Failed to get config path.");
                std::process::exit(1);
            }
        };
        
        let toml_data = fs::read_to_string(config_file)?;
        let config: Config = toml::de::from_str(&toml_data).expect("Failed to parse TOML data");
        Ok(config)
    }
}