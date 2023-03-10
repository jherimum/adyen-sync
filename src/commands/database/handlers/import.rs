use crate::database::{
    models::{NotificationItem, RawNotification, ToTarget},
    repo::{self, Pools},
};
use anyhow::{Ok, Result};
use log::warn;
use sqlx::{MySql, MySqlPool, Transaction};

pub struct Import<'a> {
    pools: &'a Pools,
    client_id: String,
    threads: u8,
}

impl<'a> Import<'a> {
    pub fn new(pools: &'a Pools, client_id: &str, threads: u8) -> Self {
        Self {
            pools,
            client_id: client_id.to_owned(),
            threads,
        }
    }

    pub async fn execute(&self, guids: &Vec<String>) -> Result<Vec<RawNotification>> {
        let mut join_handlers = vec![];
        let chunck_size = guids.len() / self.threads as usize;
        let split = guids
            .chunks(chunck_size)
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();

        println!("tamanho: {}", &split.len());

        for (ix, guids) in split.into_iter().enumerate() {
            let client_id = self.client_id.clone();
            let pools = self.pools.clone();
            let h = tokio::spawn(async move { import(pools, guids, &client_id, ix).await });
            join_handlers.push(h);
        }

        let mut output = vec![];

        for h in join_handlers {
            output.extend(h.await?.unwrap());
        }

        Ok(output)
    }
}

async fn import(
    pools: Pools,
    guids: Vec<String>,
    client_id: &str,
    ix: usize,
) -> Result<Vec<RawNotification>> {
    let mut imported_raws = vec![];
    repo::set_isolation_level(&pools.target).await?;
    repo::set_isolation_level(&pools.source).await?;
    let mut tx = pools.target.begin().await?;
    for guid in guids {
        println!("ix: {} -> Importanto guid: {}", ix, guid);
        if let Some(mut raw) = repo::find_raw_by_guid(&pools.source, &guid).await? {
            if repo::find_raw_by_guid(&mut tx, &guid).await?.is_none() {
                raw.to_target(client_id);
                repo::insert_raw_notification(&mut *tx, &raw).await?;
                fetch_and_insert_headers(&mut tx, &pools.source, &raw).await?;
                fetch_and_insert_items(&mut tx, &pools.source, &raw).await?;
            } else {
                warn!(
                    "The RawNotification with guid {} already exists on target database",
                    &guid
                );
            }
            imported_raws.push(raw);
        } else {
            warn!("RawNotification with guid {} not found", &guid);
        }
    }
    tx.commit().await?;
    println!("Commitou");

    Ok(imported_raws)
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
) -> Result<()> {
    let mut items = repo::find_items(source_pool, &raw.guid).await?;
    for item in items.iter_mut() {
        item.to_target(&raw.client_id);
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
