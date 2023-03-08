use crate::commands::database::commands::DatabaseSyncArgs;
use crate::commands::root::GlobalOpts;
use crate::database::models::RawNotification;
use crate::database::models::{NotificationItem, ToTarget};
use crate::database::repo;
use crate::database::repo::Pools;
use crate::settings::{MergeSettings, Settings};
use anyhow::{Context, Result};
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
        .context("Target clinet it not defined.")?;

    let mut max_target_raw_uid = repo::get_max_raw_uidpk(&pools.target)
        .await
        .context("Error while fetching target mas raw uidpk")?;

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
            fetch_and_insert_items(&mut tx, &pools.source, raw, &target_client_id).await?
        }
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
    dbg!(&headers);
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
