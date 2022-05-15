use serde::{Deserialize, Serialize};

use crate::common_types::{Card, CurrencyId};
use crate::payments::requests::{AdditionalInfo, BuyerIdentification};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePaymentResponse {
    pub additional_info: Option<AdditionalInfo>,
    pub card: Card,
    pub collector_id: i64,
    pub coupon_amount: i64,
    pub currency_id: CurrencyId,
    pub date_approved: String,
    pub date_created: String,
    pub date_last_updated: String,
    pub description: String,
    pub external_reference: String,
    pub fee_details: Vec<FeeDetail>,
    pub id: i64,
    pub installments: i64,
    pub issuer_id: i64,
    pub metadata: Option<serde_json::Value>,
    pub money_release_date: String,
    pub notification_url: String,
    pub order: Order,
    pub payer: BuyerIdentification,
    pub payment_method_id: String,
    pub payment_type_id: String,
    pub point_of_interaction: PointOfInteraction,
    pub processing_mode: String,
    pub shipping_amount: i64,
    pub statement_descriptor: String,
    pub status: String,
    pub status_detail: String,
    pub taxes_amount: i64,
    pub transaction_amount: f64,
    pub transaction_amount_refunded: i64,
    pub transaction_details: TransactionDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub net_received_amount: f64,
    pub total_paid_amount: f64,
    pub overpaid_amount: i64,
    pub installment_amount: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeeDetail {
    #[serde(rename = "type")]
    pub type_field: String,
    pub amount: f64,
    pub fee_payer: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfInteraction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub application_data: ApplicationData,
    pub transaction_data: TransactionData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationData {
    pub name: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionData {
    pub qr_code_base64: String,
    pub qr_code: String,
    pub ticket_url: String,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_payload_serialization() {
        let payload = serde_json::from_slice::<CreatePaymentResponse>(include_bytes!(
            "../../tests/assets/create_payment_response.json"
        ))
        .unwrap();
        println!("{:?}", payload);
    }
}
