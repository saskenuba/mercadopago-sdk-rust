//! You can setup your webhooks only

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookRequest {
    #[serde(rename = "id")]
    pub notification_id: i64,
    /// When true, indicates that it is running in production.
    pub live_mode: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub date_created: String,
    pub application_id: i64,
    /// User id of which you are receiving this notification.
    pub user_id: i64,
    pub version: i64,
    pub api_version: String,
    pub action: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "id")]
    pub payment_id: String,
}
