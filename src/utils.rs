use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use dirs::config_dir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: String::new(),
        }
    }
}

pub fn get_config_path() -> Option<PathBuf> {
    let config_dir = config_dir()?;
    let config_path = config_dir.join("hintly");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).expect("Failed to create config directory");
    }
    Some(config_path.join("config.toml"))
}

pub fn create_config_file(config_file: &Path) -> io::Result<()> {
    let config = Config::default();
    let toml_data = toml::to_string_pretty(&config).expect("Failed to serialize config to TOML");

    let mut file = File::create(config_file)?;
    file.write_all(toml_data.as_bytes())?;
    println!("Configuration file created at: {:?}", config_file);
    Ok(())
}

pub fn read_config_file(config_file: &Path) -> io::Result<Config> {
    let toml_data = fs::read_to_string(config_file)?;
    let config: Config = toml::de::from_str(&toml_data).expect("Failed to parse TOML data");
    Ok(config)
}

pub fn check_and_create_config() -> io::Result<Config> {
    let config_file = match get_config_path() {
        Some(path) => path,
        None => {
            eprintln!("Unable to find config directory.");
            return Err(io::Error::new(io::ErrorKind::NotFound, "Config directory not found"));
        }
    };

    if !config_file.exists() {
        create_config_file(&config_file)?;
        std::process::exit(0);
    } else {
        read_config_file(&config_file)
    }
}