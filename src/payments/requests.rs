use serde::{Deserialize, Serialize};

use crate::common_types::{Address, Item, PersonalIdentification, Phone, Shipments};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePaymentPayload {
    /// Description of this transaction.
    pub description: String,

    /// Required.
    pub installments: i64,

    pub order: Order,
    pub payer: BuyerIdentification,

    /// Required.
    pub payment_method_id: String,

    /// Use `MercadoPagoSDK::create_card_token` to generate one.
    /// Required for credit card payments.
    pub token: Option<String>,

    /// Total amount of the transaction
    /// Required.
    pub transaction_amount: f64,

    pub external_reference: Option<String>,

    /// Defaults to false. When set to true, payments can only be approved or rejected instantly.
    pub binary_mode: Option<bool>,

    /// Description that the payment will appear with in the card statement.
    pub statement_descriptor: Option<String>,

    pub additional_info: Option<AdditionalInfo>,

    pub metadata: Option<serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalInfo {
    pub items: Vec<Item>,
    pub payer: BuyerInformation,
    pub shipments: Shipments,
    pub barcode: Option<Barcode>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyerInformation {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<Phone>,
    pub address: Option<Address>,
    pub registration_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Barcode {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub r#type: String,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuyerEntityType {
    Individual,
    Association,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuyerType {
    Customer,
    Registered,
    Guest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyerIdentification {
    /// Required.
    pub email: Option<String>,

    #[serde(rename = "type")]
    pub buyer_type: BuyerType,
    #[serde(rename = "entity_type")]
    pub buyer_entity_type: Option<BuyerEntityType>,
    pub identification: Option<PersonalIdentification>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(
    Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::IntoStaticStr, strum::AsRefStr,
)]
#[serde(rename_all = "snake_case")]
pub enum PaymentTypeId {
    CreditCard,
    DebitCard,

    /// Instant Money transfer methods
    BankTransfer,

    /// Pay later methods, such as boleto, lotérica or another local payment
    Ticket,

    /// MercadoPago Account
    AccountMoney,
}

#[derive(
    Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::IntoStaticStr, strum::AsRefStr,
)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethodId {
    Amex,
    Visa,
    Master,
    Hipercard,

    /// Pagamento na lotérica sem boleto
    Pec,

    Pix,

    /// Boleto Bradesco
    BolBradesco,

    /// Dinheiro na conta MercadoPago
    AccountMoney,

    /// Débito Elo
    Debelo,

    /// Crédito Elo
    Elo,
}

#[derive(
    Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::IntoStaticStr, strum::AsRefStr,
)]
pub enum DocumentType {
    CPF,
    CNPJ,

    /// Argentina? Not sure since MercadoPago doesn't show from other countries (need account)
    DNI,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_payload_serialization() {
        let payload = serde_json::from_slice::<CreatePaymentPayload>(include_bytes!(
            "../../tests/assets/create_payment_request.json"
        ))
        .unwrap();
        eprintln!("payload = {:#?}", payload);
    }
}
