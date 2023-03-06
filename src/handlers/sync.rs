use anyhow::Result;

use crate::commands::sync::{SyncStatusCommand, SyncUpdateCommand, SyncWatchCommand};

pub fn status(command: SyncStatusCommand) -> Result<()> {
    Ok(())
}

pub fn update(command: SyncUpdateCommand) -> Result<()> {
    Ok(())
}

pub fn watch(command: SyncWatchCommand) -> Result<()> {
    Ok(())
}
