use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Config {
    profiles: Vec<Profile>,
}

#[derive(Deserialize, Serialize)]
struct Profile {
    name: String,
    email: String,
    signingkey: PathBuf,
    desc: String,
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

fn print_profile(profile: &Profile) {
    println!("{}", profile.name);
    println!("{}", profile.email);
    println!("{}", profile.signingkey.to_string_lossy());
    println!("{}", profile.desc);
}

fn main() -> Result<()> {
    let config_file = get_config_file()?;
    let config = parse_config_file(config_file)?;
    if config.profiles.is_empty() {
        !todo!("Ask user for creating new profile");
    } else {
        for profile in config.profiles {
            print_profile(&profile);
        }
    }

    Ok(())
}
