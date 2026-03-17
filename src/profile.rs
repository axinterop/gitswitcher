use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize, Serialize)]
pub struct Profile {
    name: String,
    email: String,
    signingkey: Option<PathBuf>,
    desc: String,
}

impl Profile {
    pub fn apply(&self) {
        self.git_config("user.name", &self.name);
        self.git_config("user.email", &self.email);

        if let Some(ref signingkey) = self.signingkey {
            self.git_config("user.signingkey", &signingkey.to_str().unwrap());
            self.git_config("gpg.format", "ssh");
            self.git_config("commit.gpgsign", "true");
            self.git_config("tag.gpgsign", "true");
        } else {
            self.git_config("user.signingkey", "");
            self.git_config("commit.gpgsign", "false");
            self.git_config("tag.gpgsign", "false");
        }
    }

    fn git_config(&self, key: &str, value: &str) {
        Command::new("git")
            .arg("config")
            .arg(key)
            .arg(value)
            .status()
            .expect("Failed to set git config");
    }
}

pub fn print_profiles(profiles: &Vec<Profile>) {
    for (i, profile) in profiles.iter().enumerate() {
        println!(
            "{0}: {1: <20} {2: <20} {3: <30} {4: <20}",
            i + 1,
            profile.name,
            profile.email,
            profile.desc,
            match &profile.signingkey {
                Some(signingkey) => signingkey.to_str(),
                None => Some("None"),
            }
            .unwrap(),
        );
    }
}

pub fn prompt_profile(profiles: &Vec<Profile>) -> Result<usize> {
    let mut choice = String::new();
    println!("Choose profile:");
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();
    let choice = choice.parse::<usize>()? - 1;
    Ok(choice)
}
