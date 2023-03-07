use crate::{
    commands::*,
    database::{self},
    settings::Settings,
};
use anyhow::Result;
use chrono::Duration as ChronoDuration;
use chrono::Utc;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use sqlx::ConnectOptions;
use sqlx::MySqlConnection;
use sqlx::{mysql::MySqlConnectOptions, MySqlPool};
use std::str::FromStr;
use std::time::Duration;

pub async fn config_show(settings: &Settings) -> Result<()> {
    println!("Settings: {}", serde_json::to_string_pretty(settings)?);
    Ok(())
}

pub async fn config_set(
    settings: &mut Settings,
    target_url: &Option<String>,
    source_url: &Option<String>,
) -> Result<()> {
    settings.update_source_url(source_url.clone());
    settings.update_target_url(target_url.clone());
    settings.write()?;
    println!("Settings: {}", serde_json::to_string_pretty(settings)?);
    Ok(())
}

pub async fn config_handler(
    settings: &mut Settings,
    globals: &GlobalOpts,
    config_command: &ConfigCommand,
) -> Result<()> {
    match &config_command.subcommand {
        ConfigSubCommand::Show => config_show(&settings).await,
        ConfigSubCommand::Set {
            target_url,
            source_url,
        } => config_set(settings, target_url, source_url).await,
    }
}

pub async fn database_handler(
    settings: &Settings,
    globals: &GlobalOpts,
    command: &DatabaseCommand,
) -> Result<()> {
    match command.subcommand {
        DatabaseSubCommand::Status => {
            database_status(&settings, &globals, &command.global_sync_opts).await
        }
        DatabaseSubCommand::Sync => {
            databse_sync(&settings, &globals, &command.global_sync_opts).await
        }
        DatabaseSubCommand::Watch => {
            database_watch(&settings, &globals, &command.global_sync_opts).await
        }
    }
}

async fn diff(source_conn: &mut MySqlConnection, target_conn: &mut MySqlConnection) -> Result<()> {
    let last = database::last_raw_notification(target_conn).await?;
    let count = database::count_raw_notification_after(
        source_conn,
        last,
        Utc::now() - ChronoDuration::minutes(1000000000),
    )
    .await?;

    println!("The target database is behind {} notifications", count);

    Ok(())
}

async fn test_connection(pool: MySqlPool, source: bool) -> Result<()> {
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

    match database::test_conn(&pool).await {
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
    globals_opts: &GlobalOpts,
    database_opts: &DatabaseOpts,
) -> Result<()> {
    let database_opts = database_opts.merge(settings);
    let pools: (MySqlPool, MySqlPool) = database_opts.try_into()?;

    test_connection(pools.0, true).await?;
    test_connection(pools.1, true).await?;

    //diff(&mut source_conn, &mut target_conn).await?;

    Ok(())
}

pub async fn databse_sync(
    cfg: &Settings,
    globals: &GlobalOpts,
    sync_globals: &DatabaseOpts,
) -> Result<()> {
    todo!()
}

pub async fn database_watch(
    cfg: &Settings,
    globals: &GlobalOpts,
    sync_globals: &DatabaseOpts,
) -> Result<()> {
    todo!()
}
