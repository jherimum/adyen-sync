use clap::Parser;
use clap::{Args, Subcommand};

use super::config::commands::ConfigCommand;
use super::database::commands::DatabaseCommand;

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
    /// Configuration commands
    Config(ConfigCommand),

    /// Database commands
    Database(DatabaseCommand),
}
