use crate::settings::Settings;
use clap::Parser;
use clap::{Args, Subcommand};

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    #[clap(short, long, global = true)]
    debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Configuration
    Config(ConfigCommand),

    /// Sync database
    Database(DatabaseCommand),
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub subcommand: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Show configurations
    Show,

    /// update configuration values
    Set {
        #[arg(short, long)]
        target_url: Option<String>,

        #[arg(short, long)]
        source_url: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct DatabaseCommand {
    #[clap(subcommand)]
    pub subcommand: DatabaseSubCommand,

    #[clap(flatten)]
    pub global_sync_opts: DatabaseOpts,
}

#[derive(Debug, Args, Clone)]
pub struct DatabaseOpts {
    #[arg(short, long, global = true)]
    pub target_url: Option<String>,

    #[arg(short, long, global = true)]
    pub source_url: Option<String>,
}

impl DatabaseOpts {
    pub fn merge(&self, settings: &Settings) -> Self {
        DatabaseOpts {
            target_url: self
                .target_url
                .as_ref()
                .or(settings.target_url.as_ref())
                .cloned(),
            source_url: self
                .source_url
                .as_ref()
                .or(settings.source_url.as_ref())
                .cloned(),
        }
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum DatabaseSubCommand {
    Status,
    Sync,
    Watch,
}
