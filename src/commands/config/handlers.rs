use anyhow::Result;

use crate::{commands::root::GlobalOpts, settings::Settings};

use super::commands::{ConfigCommand, ConfigSubCommand};

pub async fn config_handler(
    settings: &mut Settings,
    _: &GlobalOpts,
    config_command: &ConfigCommand,
) -> Result<()> {
    match &config_command.subcommand {
        ConfigSubCommand::Show => config_show(settings).await,
        ConfigSubCommand::Set {
            target_url,
            source_url,
            timeout,
        } => config_set(settings, target_url, source_url, timeout).await,
    }
}

pub async fn config_show(settings: &Settings) -> Result<()> {
    println!("Settings: {}", serde_json::to_string_pretty(settings)?);
    Ok(())
}

pub async fn config_set(
    settings: &mut Settings,
    target_url: &Option<String>,
    source_url: &Option<String>,
    timeout: &Option<i64>,
) -> Result<()> {
    settings.update_source_url(source_url);
    settings.update_target_url(target_url);
    settings.update_timeout(timeout);
    settings.write()?;
    println!("Settings: {}", serde_json::to_string_pretty(settings)?);
    Ok(())
}
