use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::mysql::MySqlConnectOptions;
use sqlx::ConnectOptions;
use std::str::FromStr;

use crate::{
    commands::*,
    database::{count_raw_notification_after, last_raw_notification},
    settings::Settings,
};

pub async fn config_show(
    cfg: &Settings,
    global: GlobalOpts,
    command: &ConfigShowCommand,
) -> Result<()> {
    println!("Settings: {}", serde_json::to_string_pretty(cfg)?);
    Ok(())
}

pub async fn config_set(cfg: &Settings, command: &ConfigSetCommand) -> Result<()> {
    let mut cfg = cfg.clone();
    cfg.update_source_url(command.source_url.clone());
    cfg.update_target_url(command.target_url.clone());

    match cfg.write() {
        Ok(_) => {
            println!("Settings: {}", serde_json::to_string_pretty(&cfg)?);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub async fn sync_status(settings: &Settings, command: &SyncStatusCommand) -> Result<()> {
    let mut target_connection =
        MySqlConnectOptions::from_str(settings.target_url.as_ref().unwrap())?
            .connect()
            .await?;
    let mut source_connection =
        MySqlConnectOptions::from_str(settings.source_url.as_ref().unwrap())?
            .connect()
            .await?;

    let last = last_raw_notification(&mut target_connection).await?;
    let count = count_raw_notification_after(
        &mut source_connection,
        last,
        Utc::now() - Duration::minutes(1),
    )
    .await?;

    println!("The target database is behind {} notifications", count);
    Ok(())
}

pub async fn sync_update(settings: &Settings, command: &SyncUpdateCommand) -> Result<()> {
    Ok(())
}

pub async fn sync_watch(settings: &Settings, command: &SyncWatchCommand) -> Result<()> {
    Ok(())
}
