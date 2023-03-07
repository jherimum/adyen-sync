use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{mysql::MySqlPoolOptions, types::BigDecimal, MySqlPool};
use std::time::Duration;

use crate::commands::DatabaseOpts;

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
