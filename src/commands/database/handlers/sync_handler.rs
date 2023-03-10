use crate::commands::database::commands::DatabaseSyncArgs;
use crate::commands::database::handlers::import::Import;
use crate::commands::root::GlobalOpts;
use crate::database::repo::Pools;
use crate::database::repo::{self};
use crate::settings::{MergeSettings, Settings};
use anyhow::{Context, Result};
use chrono::{NaiveDateTime, Utc};

pub async fn databse_sync(
    settings: &Settings,
    _: &GlobalOpts,
    args: DatabaseSyncArgs,
) -> Result<()> {
    let start = Utc::now();

    let args = args.merge(settings);
    let pools: Pools = Pools::try_from(&args).context("Error creating connection pools.")?;
    let target_client_id = args
        .target_client_id
        .context("Target client id is not defined.")?;

    let mut last_created_date = repo::get_last_raw_created_date(&pools.target)
        .await
        .context("Error while fetching target mas raw uidpk")?
        .unwrap_or(NaiveDateTime::from_timestamp_millis(0).context("invalid date")?);

    println!("Starting to sync target database.....");
    println!(
        "The last raw notification created date on target database is: {}",
        &last_created_date
    );

    let total = repo::count_raw_created_date(&pools.source, &last_created_date).await?;
    println!(
        "There are {} notifications to be imported from source database.",
        total
    );

    println!("Let's go!");

    loop {
        let guids_to_import = repo::find_raw_guid_after_created_date(
            &pools.source,
            &last_created_date,
            args.batch_size as u64,
        )
        .await?;

        if guids_to_import.is_empty() {
            break;
        }

        println!("Iniciando a importacao {}", guids_to_import.len());
        let result = Import::new(&pools, &target_client_id, args.threads)
            .execute(&guids_to_import)
            .await?;

        last_created_date = result
            .iter()
            .map(|r| r.created_date)
            .max()
            .unwrap_or(last_created_date);
    }

    print!("finish: {}", start.signed_duration_since(Utc::now()));

    Ok(())
}
