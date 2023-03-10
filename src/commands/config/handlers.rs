use anyhow::Result;

use crate::{commands::root::GlobalOpts, settings::Settings};

use super::commands::{ConfigCommand, ConfigSetArgs, ConfigSubCommand};

pub async fn config_handler(
    settings: &mut Settings,
    _: &GlobalOpts,
    config_command: ConfigCommand,
) -> Result<()> {
    match &config_command.command {
        ConfigSubCommand::Show => config_show(settings).await,
        ConfigSubCommand::Set { args } => config_set(settings, args).await,
    }
}

pub async fn config_show(settings: &Settings) -> Result<()> {
    println!("Settings: {}", serde_json::to_string_pretty(settings)?);
    Ok(())
}

pub async fn config_set(settings: &mut Settings, args: &ConfigSetArgs) -> Result<()> {
    settings.source_url(&args.source_url);
    settings.target_url(&args.target_url);
    settings.timeout(&args.timeout);
    settings.target_client_id(&args.target_client_id);
    settings.write()?;
    config_show(settings).await
}
