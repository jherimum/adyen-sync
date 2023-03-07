use chrono::NaiveDateTime;
use sqlx::types::BigDecimal;

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
