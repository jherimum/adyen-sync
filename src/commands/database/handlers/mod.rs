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
    command: &DatabaseCommand,
) -> Result<()> {
    match &command.subcommand {
        DatabaseSubCommand::Status => {
            database_status(settings, globals, &command.global_database_opts).await
        }
        DatabaseSubCommand::Watch => {
            database_watch(settings, globals, &command.global_database_opts).await
        }
        DatabaseSubCommand::Sync {
            batch_size,
            target_client_id,
        } => {
            databse_sync(
                settings,
                globals,
                &command.global_database_opts,
                *batch_size,
                target_client_id,
            )
            .await
        }
    }
}
