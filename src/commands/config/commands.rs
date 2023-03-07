use clap::{Args, Subcommand};

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
