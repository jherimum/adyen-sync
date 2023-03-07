use super::models::{
    NotificationItem, NotificationItemData, NotificationItemOperation, RawNotification,
    RawNotificationHeader,
};
use crate::commands::database::commands::DatabaseOpts;
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, types::BigDecimal, MySql, MySqlExecutor, MySqlPool};
use std::time::Duration;

const INSERT_RAW_QUERY: &str = include_str!("queries/insert_raw_notification.sql");
const INSERT_RAW_HEADER_QUERY: &str = include_str!("queries/insert_raw_notification_header.sql");

const INSERT_ITEM_QUERY: &str = include_str!("queries/insert_item.sql");
const INSERT_ITEM_DATA_QUERY: &str = include_str!("queries/insert_item_data.sql");
const INSERT_ITEM_OPERATION_QUERY: &str = include_str!("queries/insert_item_operation.sql");

const SELECT_RAW_TO_IMPORT_QUERY: &str = include_str!("queries/select_raw_to_import.sql");
const SELECT_LAST_RAW_QUERY: &str = include_str!("queries/select_last_raw_uidpk.sql");
const COUNT_RAW_TO_IMPORT_QUERY: &str = include_str!("queries/count_raw_to_import.sql");

pub async fn test_conn<'e, E: MySqlExecutor<'e>>(exec: E) -> Result<()> {
    sqlx::query_scalar::<_, i64>("select 1")
        .fetch_one(exec)
        .await
        .map(|_| ())
        .context("context")
}

pub async fn insert_raw_notification<'e, E: MySqlExecutor<'e>>(
    exec: E,
    raw: RawNotification,
) -> Result<()> {
    sqlx::query::<MySql>(INSERT_RAW_QUERY)
        .bind(raw.uidpk)
        .bind(raw.guid)
        .bind(raw.created_date)
        .bind(raw.consumed_date)
        .bind(raw.consumed)
        .bind(raw.body)
        .bind(raw.client_id)
        .bind(raw.consume_success)
        .execute(exec)
        .await
        .context("context")?;
    Ok(())
}

pub async fn insert_raw_notification_header<'e, E: MySqlExecutor<'e>>(
    exec: E,
    header: RawNotificationHeader,
) -> Result<()> {
    sqlx::query::<MySql>(INSERT_RAW_HEADER_QUERY)
        .bind(header.tadyen_raw_notification_uid)
        .bind(header.name)
        .bind(header.value)
        .execute(exec)
        .await
        .context("context")?;
    Ok(())
}

pub async fn insert_item_data<'e, E: MySqlExecutor<'e>>(
    exec: E,
    data: NotificationItemData,
) -> Result<()> {
    sqlx::query::<MySql>(INSERT_ITEM_DATA_QUERY)
        .bind(data.notification_item_uid)
        .bind(data.name)
        .bind(data.value)
        .execute(exec)
        .await
        .context("context")?;
    Ok(())
}

pub async fn insert_item_operation<'e, E: MySqlExecutor<'e>>(
    exec: E,
    operation: NotificationItemOperation,
) -> Result<()> {
    sqlx::query::<MySql>(INSERT_ITEM_OPERATION_QUERY)
        .bind(operation.notification_item_uid)
        .bind(operation.operation)
        .execute(exec)
        .await
        .context("context")?;
    Ok(())
}

pub async fn insert_item<'e, E: MySqlExecutor<'e>>(exec: E, item: NotificationItem) -> Result<()> {
    sqlx::query::<MySql>(INSERT_ITEM_QUERY)
        .bind(item.uidpk)
        .bind(item.guid)
        .bind(item.created_date)
        .bind(item.consume_success)
        .bind(item.consumed_date)
        .bind(item.consumed)
        .bind(item.currency)
        .bind(item.amount)
        .bind(item.event_code)
        .bind(item.event_date)
        .bind(item.merchant_account_code)
        .bind(item.merchant_reference)
        .bind(item.payment_method)
        .bind(item.psp_reference)
        .bind(item.reason)
        .bind(item.success)
        .bind(item.live)
        .bind(item.original_reference)
        .bind(item.client_id)
        .bind(item.raw_notification_item_guid)
        .execute(exec)
        .await
        .context("context")?;
    Ok(())
}

pub async fn retrieve_raw_notification<'e, E: MySqlExecutor<'e>>(
    exec: E,
    after: &BigDecimal,
    batch_size: i64,
) -> Result<Vec<RawNotification>> {
    sqlx::query_as::<_, RawNotification>(SELECT_RAW_TO_IMPORT_QUERY)
        .bind(after)
        .bind(batch_size)
        .fetch_all(exec)
        .await
        .context("context")
}

pub async fn last_raw_notification<'e, E: MySqlExecutor<'e>>(exec: E) -> Result<BigDecimal> {
    sqlx::query_scalar::<_, BigDecimal>(SELECT_LAST_RAW_QUERY)
        .fetch_one(exec)
        .await
        .context("context")
}

pub async fn count_raw_notification_after<'e, E: MySqlExecutor<'e>>(
    exec: E,
    uidpk: &BigDecimal,
) -> Result<i64> {
    sqlx::query_scalar::<_, i64>(COUNT_RAW_TO_IMPORT_QUERY)
        .bind(uidpk)
        .fetch_one(exec)
        .await
        .context("context")
}

impl TryFrom<DatabaseOpts> for (MySqlPool, MySqlPool) {
    type Error = anyhow::Error;

    fn try_from(value: DatabaseOpts) -> Result<Self, Self::Error> {
        let source = MySqlPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect_lazy(&value.source_url.unwrap())?;
        let target = MySqlPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .connect_lazy(&value.target_url.unwrap())?;
        Ok((source, target))
    }
}