use std::env;

use adyen_sync::{
    commands::{config::ConfigCommand, sync::SyncCommand},
    handlers::{
        config::{set, show},
        sync::{status, update, watch},
    },
    settings::Settings,
};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct AdyenSyncArgs {
    #[command(subcommand)]
    commands: AdyenSyncCommand,

    #[arg(short, long, global = true)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum AdyenSyncCommand {
    /// Configuration
    Config(ConfigCommand),

    /// Sync database
    Sync(SyncCommand),
}

fn main() -> Result<(), anyhow::Error> {
    let config = Settings::load()?;
    let args = AdyenSyncArgs::parse();

    match args.commands {
        AdyenSyncCommand::Config(config_c) => match config_c.commands {
            adyen_sync::commands::config::ConfigSubCommand::Show(c) => show(&config, &c),
            adyen_sync::commands::config::ConfigSubCommand::Set(c) => set(&config, &c),
        },
        AdyenSyncCommand::Sync(sync_c) => match sync_c.commands {
            adyen_sync::commands::sync::SyncSubCommand::Status(c) => status(c),
            adyen_sync::commands::sync::SyncSubCommand::Update(c) => update(c),
            adyen_sync::commands::sync::SyncSubCommand::Watch(c) => watch(c),
        },
    }
}
