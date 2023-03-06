use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub commands: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Show configurations
    Show(ConfigShowCommand),

    /// update configuration values
    Set(ConfigSetCommand),
}

#[derive(Debug, Args)]
pub struct ConfigShowCommand {}

#[derive(Debug, Args)]
pub struct ConfigSetCommand {
    #[arg(short, long)]
    pub target_url: Option<String>,

    #[arg(short, long)]
    pub source_url: Option<String>,
}
