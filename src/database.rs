use std::time::Duration;

use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{mysql::MySqlPoolOptions, query_scalar, types::BigDecimal, MySqlConnection, MySqlPool};

use crate::commands::DatabaseOpts;

pub struct RawNotification {
    pub UIDPK: BigDecimal,
    pub GUID: String,
    pub CREATED_DATE: NaiveDateTime,
    pub CONSUMED_DATE: Option<NaiveDateTime>,
    pub CONSUMED: BigDecimal,
    pub BODY: Option<String>,
    pub CLIENT_ID: String,
    pub CONSUME_SUCCESS: BigDecimal,
}

pub struct RawNotificationHeader {
    TADYEN_RAW_NOTIFICATION_UID: BigDecimal,
    NAME: String,
    VALUE: Option<String>,
}

pub struct NotificationItem {
    UIDPK: BigDecimal,
    GUID: String,
    CREATED_DATE: NaiveDateTime,
    CONSUME_SUCCESS: BigDecimal,
    CONSUMED_DATE: Option<NaiveDateTime>,
    CONSUMED: BigDecimal,
    CURRENCY: Option<String>,
    AMOUNT: Option<BigDecimal>,
    EVENT_CODE: String,
    EVENT_DATE: Option<NaiveDateTime>,
    MERCHANT_ACCOUNT_CODE: String,
    MERCHANT_REFERENCE: String,
    PAYMENT_METHOD: String,
    PSP_REFERENCE: String,
    REASON: Option<String>,
    SUCCESS: BigDecimal,
    LIVE: BigDecimal,
    ORIGINAL_REFERENCE: String,
    CLIENT_ID: String,
    RAW_NOTIFICATION_ITEM_GUID: String,
}

pub struct NotificationItemOperation {
    NOTIFICATION_ITEM_UID: BigDecimal,
    OPERATION: String,
}

pub struct NotificationItemData {
    NOTIFICATION_ITEM_UID: BigDecimal,
    NAME: String,
    VALUE: Option<String>,
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
