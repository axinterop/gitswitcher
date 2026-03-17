mod configuration;
mod profile;

use anyhow::Result;

use crate::configuration::Config;

fn main() -> Result<()> {
    let config = Config::new();
    let profiles = config?.get_profiles();
    if profiles.is_empty() {
        !todo!("Ask user for creating new profile");
    } else {
        profile::print_profiles(&profiles);
        let profile_num = profile::prompt_profile(&profiles)?;
        if let Some(profile) = profiles.get(profile_num) {
            profile.apply();
        } else {
            panic!("Profile out of bounds");
        }
    }

    Ok(())
}
