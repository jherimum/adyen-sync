use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct SyncCommand {
    #[clap(subcommand)]
    pub commands: SyncSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SyncSubCommand {
    Status(SyncStatusCommand),
    Update(SyncUpdateCommand),
    Watch(SyncWatchCommand),
}

#[derive(Debug, Args)]
pub struct SyncStatusCommand {}

#[derive(Debug, Args)]
pub struct SyncUpdateCommand {}

#[derive(Debug, Args)]
pub struct SyncWatchCommand {}
