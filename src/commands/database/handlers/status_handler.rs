use crate::commands::database::commands::DatabaseStatusArgs;
use crate::commands::root::GlobalOpts;
use crate::database::repo::count_raw_notification_after;
use crate::database::repo::get_max_raw_uidpk;
use crate::database::repo::test_conn;
use crate::database::repo::Pools;
use crate::settings::MergeSettings;
use crate::settings::Settings;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::MySqlPool;
use std::time::Duration;

pub async fn database_status(
    settings: &Settings,
    _: &GlobalOpts,
    args: DatabaseStatusArgs,
) -> Result<()> {
    let args = args.merge(settings);
    let pools: Pools = Pools::try_from(&args)?;

    test_connection(&pools.source, true).await?;
    test_connection(&pools.target, true).await?;

    diff(&pools.source, &pools.target).await?;

    Ok(())
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
