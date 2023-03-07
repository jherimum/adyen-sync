use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{types::BigDecimal, MySqlConnection};

pub struct RawNotification {
    UIDPK: BigDecimal,
    GUID: String,
    CREATED_DATE: NaiveDateTime,
    CONSUMED_DATE: Option<NaiveDateTime>,
    CONSUMED: BigDecimal,
    BODY: Option<String>,
    CLIENT_ID: String,
    CONSUME_SUCCESS: BigDecimal,
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

pub async fn last_raw_notification(conn: &mut MySqlConnection) -> Result<BigDecimal> {
    sqlx::query_scalar::<_, BigDecimal>(
        "select coalesce(0, max(uidpk)) from tadyen_raw_notification",
    )
    .fetch_one(conn)
    .await
    .context("context")
}

pub async fn count_raw_notification_after(
    conn: &mut MySqlConnection,
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
