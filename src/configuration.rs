use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::fs::{self, File};
use std::io::{Read, Write};

use crate::profile::Profile;

#[derive(Deserialize, Serialize)]
pub struct Config {
    profiles: Vec<Profile>,
}

impl Config {
    pub fn new() -> Result<Self> {
        let config_file = Self::get_config_file()?;
        Self::parse_config_file(config_file)
    }

    pub fn get_profiles(self) -> Vec<Profile> {
        return self.profiles;
    }

    fn get_config_file() -> Result<File> {
        let crate_name = env!("CARGO_PKG_NAME");
        let config_filename = "config.toml";
        let config_dir = dirs::config_dir().unwrap().join(crate_name);
        let config_file = config_dir.join(config_filename);

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
            println!("Created config directory: {:?}", config_dir);
        }

        if !config_file.exists() {
            let mut file = fs::File::create(&config_file)?;
            writeln!(
                file,
                "# Configuration file for {} (created automatically)\n",
                crate_name
            )?;
            println!("Created config file: {:?}", config_file);
        } else {
            println!("Opening existing config file: {:?}", config_file);
        };

        Ok(fs::File::open(&config_file)?)
    }

    fn parse_config_file(mut config_file: File) -> Result<Config> {
        let mut config_buf = String::new();
        config_file.read_to_string(&mut config_buf)?;
        Ok(toml::from_str(&config_buf)?)
    }
}
