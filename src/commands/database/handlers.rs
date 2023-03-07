use std::io;
use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::MySqlPool;

use crate::commands::root::GlobalOpts;
use crate::database::models::ToTarget;
use crate::database::repo;
use crate::database::repo::count_raw_notification_after;
use crate::database::repo::get_max_raw_uidpk;
use crate::database::repo::test_conn;
use crate::database::repo::Pools;
use crate::settings::Settings;

use super::commands::DatabaseCommand;
use super::commands::DatabaseOpts;
use super::commands::DatabaseSubCommand;

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
                target_client_id.clone(),
            )
            .await
        }
    }
}

async fn diff(source_conn: &MySqlPool, target_conn: &MySqlPool) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );

    spinner.set_message(
        "Calculating the number of notifications are not sync with target database...",
    );

    let last = get_max_raw_uidpk(target_conn).await?;
    let count = count_raw_notification_after(source_conn, &last).await?;

    spinner.finish_with_message(format!(
        "There are {} notifications not sync on target database.",
        count
    ));

    Ok(())
}

async fn test_connection(pool: &MySqlPool, source: bool) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    let conn_type = if source { "source" } else { "target" };
    spinner.set_message(format!("Verifyng and test {} connection...", conn_type));

    match test_conn(pool).await {
        Ok(_) => {
            spinner.finish_with_message(format!("{} connection verified successfully", conn_type));
            Ok(())
        }
        Err(e) => {
            spinner.finish_with_message(format!("{} connection verification failed", conn_type));
            Err(e)
        }
    }
}

pub async fn database_status(
    settings: &Settings,
    _: &GlobalOpts,
    database_opts: &DatabaseOpts,
) -> Result<()> {
    let database_opts = database_opts.merge(settings);
    let pools: Pools = database_opts.try_into()?;

    test_connection(&pools.source, true).await?;
    test_connection(&pools.target, true).await?;

    diff(&pools.source, &pools.target).await?;

    Ok(())
}

pub async fn databse_sync(
    settings: &Settings,
    global_opts: &GlobalOpts,
    database_opts: &DatabaseOpts,
    batch_size: u8,
    target_client_id: String,
) -> Result<()> {
    let database_opts = database_opts.merge(settings);
    let pools: Pools = database_opts.try_into()?;

    let mut max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target).await?;
    let mut raws_to_import =
        repo::find_raw_after_uidpk(&pools.source, &max_target_raw_uid, batch_size).await?;

    while !raws_to_import.is_empty() {
        let mut tx = pools.target.begin().await?;
        for r in raws_to_import.iter_mut() {
            r.to_target(&target_client_id);
            repo::insert_raw_notification(&mut tx, r).await?;
        }
        tx.commit().await?;

        max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target).await?;
        raws_to_import =
            repo::find_raw_after_uidpk(&pools.source, &max_target_raw_uid, batch_size).await?;
    }

    Ok(())
}

pub async fn database_watch(_: &Settings, _: &GlobalOpts, _: &DatabaseOpts) -> Result<()> {
    todo!()
}
