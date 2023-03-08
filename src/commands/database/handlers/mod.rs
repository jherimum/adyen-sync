use anyhow::Result;

use crate::{commands::root::GlobalOpts, settings::Settings};

use self::{
    status_handler::database_status, sync_handler::databse_sync, watch_handler::database_watch,
};

use super::commands::{DatabaseCommand, DatabaseSubCommand};

pub mod status_handler;
pub mod sync_handler;
pub mod watch_handler;

pub async fn database_handler(
    settings: &Settings,
    globals: &GlobalOpts,
    command: DatabaseCommand,
) -> Result<()> {
    match command.command {
        DatabaseSubCommand::Status { args } => database_status(settings, globals, args).await,
        DatabaseSubCommand::Watch => database_watch(settings, globals).await,
        DatabaseSubCommand::Sync { args } => databse_sync(settings, globals, args).await,
    }
}
