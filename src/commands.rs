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
    /// Configuration commands
    Config(ConfigCommand),

    /// Database commands
    Database(DatabaseCommand),
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub subcommand: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Show configuration values
    Show,

    /// Update configuration values
    Set {
        /// Target mysql database connection String. Ex: mysql://user:password@host:port/database
        #[arg(short, long)]
        target_url: Option<String>,

        /// Source mysql database connection String. Ex: mysql://user:password@host:port/database
        #[arg(short, long)]
        source_url: Option<String>,

        /// Timeout in seconds t aquire a connection
        #[arg(short, long)]
        timeout: Option<i64>,
    },
}

#[derive(Debug, Args)]
pub struct DatabaseCommand {
    #[clap(subcommand)]
    pub subcommand: DatabaseSubCommand,

    #[clap(flatten)]
    pub global_database_opts: DatabaseOpts,
}

#[derive(Debug, Args, Clone)]
pub struct DatabaseOpts {
    /// Target mysql database connection String. Ex: mysql://user:password@host:port/database
    #[arg(short, long, global = true)]
    pub target_url: Option<String>,

    /// Source mysql database connection String. Ex: mysql://user:password@host:port/database
    #[arg(short, long, global = true)]
    pub source_url: Option<String>,

    /// Timeout in seconds t aquire a connection
    #[arg(short, long, global = true)]
    pub timeout: Option<i64>,
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
            timeout: self.timeout.as_ref().or(settings.timeout.as_ref()).cloned(),
        }
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum DatabaseSubCommand {
    /// Verify and test target and source connections and show how many notifications are not sync
    Status,

    /// Sync notifications
    Sync,

    /// Watch source database updates and sync
    Watch,
}
