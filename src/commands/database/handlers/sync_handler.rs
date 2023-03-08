use std::fmt::Write;

use crate::commands::database::commands::DatabaseSyncArgs;
use crate::commands::root::GlobalOpts;
use crate::database::models::RawNotification;
use crate::database::models::{NotificationItem, ToTarget};
use crate::database::repo::Pools;
use crate::database::repo::{self, count_raw_notification_after};
use crate::settings::{MergeSettings, Settings};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::info;
use sqlx::{MySql, MySqlPool, Transaction};

pub async fn databse_sync(
    settings: &Settings,
    _: &GlobalOpts,
    args: DatabaseSyncArgs,
) -> Result<()> {
    let args = args.merge(settings);
    let pools: Pools = Pools::try_from(&args).context("Error creating connection pools.")?;
    let target_client_id = args
        .target_client_id
        .context("Target client id is not defined.")?;

    let mut max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target)
        .await
        .context("Error while fetching target mas raw uidpk")?;

    let total_to_import = count_raw_notification_after(&pools.source, &max_target_raw_uid).await?;

    println!("Starting to sync target database.....");
    println!(
        "The last raw notification uidpk on target database is: {}",
        &max_target_raw_uid
    );
    println!(
        "There are {} notifications to be imported from source database.",
        &total_to_import
    );

    println!("Let's go!");

    let sync_pb = ProgressBar::new(total_to_import as u64);
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {human_pos}/{human_len} {msg} {eta}",
    )
    .unwrap()
    .progress_chars("##-");

    sync_pb.set_style(sty);

    let mut raws_to_import =
        repo::find_raw_after_uidpk(&pools.source, &max_target_raw_uid, args.batch_size)
            .await
            .context("Erro while fetching raw notifications to import")?;

    while !raws_to_import.is_empty() {
        let mut tx = pools.target.begin().await?;
        for raw in raws_to_import.iter_mut() {
            raw.to_target(&target_client_id);
            repo::insert_raw_notification(&mut tx, raw).await?;
            fetch_and_insert_headers(&mut tx, &pools.source, raw).await?;
            fetch_and_insert_items(&mut tx, &pools.source, raw, &target_client_id).await?;

            println!("Raw notification {} imported!", &raw.uidpk);
            // sync_pb.inc(1);
            // sync_pb.println(format!("Raw notification {} imported!", &raw.uidpk));
        }
        // sync_pb.println("Commit done!");
        tx.commit().await?;

        max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target).await?;
        raws_to_import =
            repo::find_raw_after_uidpk(&pools.source, &max_target_raw_uid, args.batch_size).await?;
    }

    Ok(())
}

async fn fetch_and_insert_headers(
    tx: &mut Transaction<'_, MySql>,
    source_pool: &MySqlPool,
    raw: &RawNotification,
) -> Result<()> {
    let headers = repo::find_headers(source_pool, &raw.uidpk).await?;
    for header in headers {
        repo::insert_raw_notification_header(&mut *tx, &header).await?;
    }
    Ok(())
}

async fn fetch_and_insert_items(
    tx: &mut Transaction<'_, MySql>,
    source_pool: &MySqlPool,
    raw: &RawNotification,
    client_id: &str,
) -> Result<()> {
    let mut items = repo::find_items(source_pool, &raw.guid).await?;
    for item in items.iter_mut() {
        item.to_target(client_id);
        repo::insert_item(&mut *tx, item).await?;
        fetch_and_insert_item_data(&mut *tx, source_pool, item).await?;
        fetch_and_insert_item_operation(&mut *tx, source_pool, item).await?;
    }
    Ok(())
}

async fn fetch_and_insert_item_data(
    tx: &mut Transaction<'_, MySql>,
    source_pool: &MySqlPool,
    item: &NotificationItem,
) -> Result<()> {
    let datas = repo::find_item_data(source_pool, &item.uidpk).await?;
    for data in datas {
        repo::insert_item_data(&mut *tx, &data).await?;
    }
    Ok(())
}

async fn fetch_and_insert_item_operation(
    tx: &mut Transaction<'_, MySql>,
    source_pool: &MySqlPool,
    item: &NotificationItem,
) -> Result<()> {
    let operations = repo::find_item_operations(source_pool, &item.uidpk).await?;
    for operation in operations {
        repo::insert_item_operation(&mut *tx, &operation).await?;
    }
    Ok(())
}
