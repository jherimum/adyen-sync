use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub command: ConfigSubCommand,
}

#[derive(Debug, Args)]
pub struct ConfigSetArgs {
    /// Target mysql database connection String. Ex: mysql://user:password@host:port/database
    #[arg(short('t'), long)]
    pub target_url: Option<String>,

    /// Source mysql database connection String. Ex: mysql://user:password@host:port/database
    #[arg(short('s'), long)]
    pub source_url: Option<String>,

    /// Timeout in seconds t aquire a connection
    #[arg(short('o'), long)]
    pub timeout: Option<u64>,

    /// Target client id
    #[arg(short('c'), long)]
    pub target_client_id: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Show configuration values
    Show,

    /// Update configuration values
    Set {
        #[clap(flatten)]
        args: ConfigSetArgs,
    },
}
