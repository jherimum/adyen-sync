use crate::settings::{MergeSettings, Settings};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DatabaseCommand {
    #[clap(subcommand)]
    pub command: DatabaseSubCommand,
}

#[derive(Debug, Args, Clone)]
pub struct CommonsDatabaseArgs {
    /// Target mysql database connection String. Ex: mysql://user:password@host:port/database
    #[arg(short = 't', long, global = true)]
    pub target_url: Option<String>,

    /// Source mysql database connection String. Ex: mysql://user:password@host:port/database
    #[arg(short = 's', long, global = true)]
    pub source_url: Option<String>,

    /// Timeout in seconds t aquire a connection
    #[arg(short = 'o', long, global = true)]
    pub timeout: Option<u64>,
}

impl MergeSettings for CommonsDatabaseArgs {
    fn merge(self, settings: &Settings) -> Self {
        Self {
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
    Status {
        #[clap(flatten)]
        args: DatabaseStatusArgs,
    },

    /// Sync notifications
    Sync {
        #[clap(flatten)]
        args: DatabaseSyncArgs,
    },

    /// Watch source database updates and sync
    Watch {
        #[clap(flatten)]
        args: DatabaseWatchArgs,
    },
}

#[derive(Debug, Args, Clone)]
pub struct DatabaseSyncArgs {
    #[clap(flatten)]
    pub args: CommonsDatabaseArgs,

    ///batch size dor sync database
    #[arg(short, long, default_value_t = 200)]
    pub batch_size: usize,

    /// Client id to be used on target database
    #[arg(short, long)]
    pub target_client_id: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    pub threads: u8,
}

impl MergeSettings for DatabaseSyncArgs {
    fn merge(self, settings: &Settings) -> Self {
        DatabaseSyncArgs {
            args: self.args.merge(&settings),
            batch_size: self.batch_size,
            target_client_id: self.target_client_id.or(settings.target_client_id.clone()),
            threads: self.threads,
        }
    }
}

impl MergeSettings for DatabaseStatusArgs {
    fn merge(self, settings: &Settings) -> Self {
        DatabaseStatusArgs {
            common_args: self.common_args.merge(settings),
        }
    }
}

#[derive(Debug, Args, Clone)]
pub struct DatabaseStatusArgs {
    #[clap(flatten)]
    pub common_args: CommonsDatabaseArgs,
}

#[derive(Debug, Args, Clone)]
pub struct DatabaseWatchArgs {
    #[clap(flatten)]
    pub common_args: CommonsDatabaseArgs,

    /// pooling delay in seconds. Default: 5 seconds
    #[arg(short, long, default_value_t = 5)]
    pub delay: u8,

    /// batch soze for each loop
    #[arg(short, long, default_value_t = 10)]
    pub batch_size: u8,

    #[arg(short, long)]
    pub target_client_id: Option<String>,
}

impl MergeSettings for DatabaseWatchArgs {
    fn merge(self, settings: &Settings) -> Self {
        DatabaseWatchArgs {
            common_args: self.common_args.merge(settings),
            delay: self.delay,
            batch_size: self.batch_size,
            target_client_id: self.target_client_id.or(settings.target_client_id.clone()),
        }
    }
}
