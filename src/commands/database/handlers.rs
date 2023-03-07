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
use crate::database::repo::last_raw_notification;
use crate::database::repo::test_conn;
use crate::settings::Settings;

use super::commands::DatabaseCommand;
use super::commands::DatabaseOpts;
use super::commands::DatabaseSubCommand;

pub async fn database_handler(
    settings: &Settings,
    globals: &GlobalOpts,
    command: &DatabaseCommand,
) -> Result<()> {
    match command.subcommand {
        DatabaseSubCommand::Status => {
            database_status(settings, globals, &command.global_database_opts).await
        }
        DatabaseSubCommand::Sync => {
            databse_sync(settings, globals, &command.global_database_opts).await
        }
        DatabaseSubCommand::Watch => {
            database_watch(settings, globals, &command.global_database_opts).await
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

    let last = last_raw_notification(target_conn).await?;
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
    let pools: (MySqlPool, MySqlPool) = database_opts.try_into()?;

    test_connection(&pools.0, true).await?;
    test_connection(&pools.1, true).await?;

    diff(&pools.0, &pools.1).await?;

    Ok(())
}

pub async fn databse_sync(
    settings: &Settings,
    global_opts: &GlobalOpts,
    database_opts: &DatabaseOpts,
) -> Result<()> {
    let database_opts = database_opts.merge(settings);
    let pools: (MySqlPool, MySqlPool) = database_opts.try_into()?;

    let mut last_uid = repo::last_raw_notification(&pools.1).await?;
    //let count = queries::count_raw_notification_after(&pools.1, &last_uid).await?;
    let mut result = repo::retrieve_raw_notification(&pools.0, &last_uid, 10).await?;

    while !result.is_empty() {
        let mut tx = pools.1.begin().await?;
        for r in result.iter_mut() {
            r.to_target("client_id");
            repo::insert_raw_notification(&mut tx, r.clone()).await?;
            println!("{}", r.uidpk);
            io::stdout().flush().unwrap();
        }

        tx.commit().await?;
        last_uid = repo::last_raw_notification(&pools.1).await?;
        result = repo::retrieve_raw_notification(&pools.0, &last_uid, 10).await?;
    }

    Ok(())
}

pub async fn database_watch(_: &Settings, _: &GlobalOpts, _: &DatabaseOpts) -> Result<()> {
    todo!()
}
