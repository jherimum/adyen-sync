use anyhow::Result;

use crate::{
    commands::config::{ConfigSetCommand, ConfigShowCommand},
    settings::Settings,
};

pub fn show(cfg: &Settings, command: &ConfigShowCommand) -> Result<()> {
    println!("Settings: {}", serde_json::to_string_pretty(cfg)?);
    Ok(())
}

pub fn set(cfg: &Settings, command: &ConfigSetCommand) -> Result<()> {
    let mut cfg = cfg.clone();
    cfg.update_source_url(command.source_url.clone());
    cfg.update_target_url(command.target_url.clone());

    match cfg.write() {
        Ok(_) => {
            println!("Settings: {}", serde_json::to_string_pretty(&cfg)?);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
