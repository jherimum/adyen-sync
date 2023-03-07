use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{mysql::MySqlPoolOptions, types::BigDecimal, MySqlPool};

use crate::commands::DatabaseOpts;

pub struct RawNotification {
    pub uidpk: BigDecimal,
    pub guid: String,
    pub created_date: NaiveDateTime,
    pub consumed_date: Option<NaiveDateTime>,
    pub consumed: BigDecimal,
    pub body: Option<String>,
    pub client_id: String,
    pub consume_success: BigDecimal,
}

pub struct RawNotificationHeader {
    pub tadyen_raw_notification_uid: BigDecimal,
    pub name: String,
    pub value: Option<String>,
}

pub struct NotificationItem {
    pub uidpk: BigDecimal,
    pub guid: String,
    pub created_date: NaiveDateTime,
    pub consume_success: BigDecimal,
    pub consumed_date: Option<NaiveDateTime>,
    pub consumed: BigDecimal,
    pub currency: Option<String>,
    pub amount: Option<BigDecimal>,
    pub event_code: String,
    pub event_date: Option<NaiveDateTime>,
    pub merchant_account_code: String,
    pub merchant_reference: String,
    pub payment_method: String,
    pub psp_reference: String,
    pub reason: Option<String>,
    pub success: BigDecimal,
    pub live: BigDecimal,
    pub original_reference: String,
    pub client_id: String,
    pub raw_notification_item_guid: String,
}

pub struct NotificationItemOperation {
    pub notification_item_uid: BigDecimal,
    pub operation: String,
}

pub struct NotificationItemData {
    pub notification_item_uid: BigDecimal,
    pub name: String,
    pub value: Option<String>,
}

pub async fn test_conn(conn: &MySqlPool) -> Result<()> {
    sqlx::query_scalar::<_, i64>("select 1")
        .fetch_one(conn)
        .await
        .map(|_| ())
        .context("context")
}

pub async fn last_raw_notification(conn: &MySqlPool) -> Result<BigDecimal> {
    sqlx::query_scalar::<_, BigDecimal>(
        "select coalesce(0, max(uidpk)) from tadyen_raw_notification",
    )
    .fetch_one(conn)
    .await
    .context("context")
}

pub async fn count_raw_notification_after(
    conn: &MySqlPool,
    uidpk: BigDecimal,
    after: DateTime<Utc>,
) -> Result<i64> {
    sqlx::query_scalar::<_, i64>(
        "select count(1) from tadyen_raw_notification n where n.uidpk > ? and n.created_date > ?",
    )
    .bind(uidpk)
    .bind(after)
    .fetch_one(conn)
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
