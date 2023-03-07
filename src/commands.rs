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
    Sync(SyncCommand),
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub command: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Show configurations
    Show(ConfigShowCommand),

    /// update configuration values
    Set(ConfigSetCommand),
}

#[derive(Debug, Args, Clone)]
pub struct ConfigShowCommand {}

#[derive(Debug, Args, Clone)]
pub struct ConfigSetCommand {
    #[arg(short, long)]
    pub target_url: Option<String>,

    #[arg(short, long)]
    pub source_url: Option<String>,
}

#[derive(Debug, Args, Clone)]
pub struct SyncCommand {
    #[clap(subcommand)]
    pub commands: SyncSubCommand,
    //Minutes behind from now
}

#[derive(Debug, Args, Clone)]
pub struct GlobalSync {
    #[arg(short, long, global = true, default_value_t = 10)]
    pub after: i64,

    #[arg(short, long, global = true)]
    pub target_url: String,

    #[arg(short, long, global = true)]
    pub source_url: String,
}

#[derive(Debug, Subcommand, Clone)]
pub enum SyncSubCommand {
    Status(SyncStatusCommand),
    Update(SyncUpdateCommand),
    Watch(SyncWatchCommand),
}

#[derive(Debug, Args, Clone)]
pub struct SyncStatusCommand {}

#[derive(Debug, Args, Clone)]
pub struct SyncUpdateCommand {}

#[derive(Debug, Args, Clone)]
pub struct SyncWatchCommand {}
