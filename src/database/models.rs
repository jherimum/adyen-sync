use std::fmt::Debug;

use chrono::{NaiveDateTime, Utc};
use sqlx::{types::BigDecimal, FromRow};

pub trait ToTarget {
    fn to_target(&mut self, client_id: &str);
}

#[derive(FromRow, Clone)]
#[sqlx(rename_all = "UPPERCASE")]
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

impl Debug for RawNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawNotification")
            .field("uidpk", &self.uidpk)
            .field("guid", &self.guid)
            .field("created_date", &self.created_date)
            .field("consumed_date", &self.consumed_date)
            .field("consumed", &self.consumed)
            .field("client_id", &self.client_id)
            .field("consume_success", &self.consume_success)
            .finish()
    }
}

impl ToTarget for RawNotification {
    fn to_target(&mut self, client_id: &str) {
        self.consumed = BigDecimal::from(0);
        self.consumed_date = None;
        self.consume_success = BigDecimal::from(0);
        self.client_id = client_id.to_owned();
    }
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "UPPERCASE")]
pub struct RawNotificationHeader {
    pub tadyen_raw_notification_uid: BigDecimal,
    pub name: String,
    pub value: Option<String>,
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "UPPERCASE")]
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
    pub payment_method: Option<String>,
    pub psp_reference: String,
    pub reason: Option<String>,
    pub success: BigDecimal,
    pub live: BigDecimal,
    pub original_reference: Option<String>,
    pub client_id: String,
    pub raw_notification_item_guid: String,
}

impl ToTarget for NotificationItem {
    fn to_target(&mut self, client_id: &str) {
        self.consumed_date = None;
        self.client_id = client_id.to_owned();
        self.consume_success = BigDecimal::from(0);
        self.consumed = BigDecimal::from(0);
    }
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "UPPERCASE")]
pub struct NotificationItemOperation {
    pub notification_item_uid: BigDecimal,
    pub operation: String,
}

#[derive(FromRow, Debug)]
#[sqlx(rename_all = "UPPERCASE")]
pub struct NotificationItemData {
    pub notification_item_uid: BigDecimal,
    pub name: String,
    pub value: Option<String>,
}
