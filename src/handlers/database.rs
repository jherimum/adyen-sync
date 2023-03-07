use std::time::Duration;

use anyhow::Result;
use chrono::Duration as ChronoDuration;
use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::MySqlPool;

use crate::commands::DatabaseOpts;
use crate::{
    commands::{DatabaseCommand, DatabaseSubCommand, GlobalOpts},
    database,
    settings::Settings,
};

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

    let last = database::last_raw_notification(target_conn).await?;
    let count = database::count_raw_notification_after(
        source_conn,
        last,
        Utc::now() - ChronoDuration::minutes(1000000000),
    )
    .await?;

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

    match database::test_conn(pool).await {
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

pub async fn databse_sync(_: &Settings, _: &GlobalOpts, _: &DatabaseOpts) -> Result<()> {
    todo!()
}

pub async fn database_watch(_: &Settings, _: &GlobalOpts, _: &DatabaseOpts) -> Result<()> {
    todo!()
}
