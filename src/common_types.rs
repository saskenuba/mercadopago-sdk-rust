use serde::{Deserialize, Serialize};

use crate::errors::ValidationError;
use crate::helpers::option_stringify;
use crate::payments::requests::DocumentType;
use crate::SDKError;

/// ID of the currency used in the payment accepted by MercadoPago
/// ARS: Argentine peso.
/// BRL: Brazilian real.
/// CLP: Chilean peso.
/// MXN: Mexican peso.
/// COP: Colombian peso.
/// PEN: Peruvian sol.
/// UYU: Uruguayan peso.
#[derive(
    Copy, Clone, Deserialize, Serialize, PartialEq, Debug, strum::IntoStaticStr, strum::AsRefStr,
)]
pub enum CurrencyId {
    ARS,
    BRL,
    CLP,
    MXN,
    COP,
    PEN,
    UYU,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phone {
    #[serde(
        default,
        deserialize_with = "serde_aux::field_attributes::deserialize_option_number_from_string"
    )]
    pub area_code: Option<i64>,
    pub number: Option<String>,
}

/// Redirection URLs after customer pays.
///
/// Used by the Checkout and CheckoutPro.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackUrls {
    pub success: Option<String>,
    pub failure: Option<String>,
    pub pending: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMethods {
    /// Method that excludes specific credit and debit card brands, such as Visa, Mastercard,
    /// American Express, among others. Note that these are DEBIT/CREDIT CARD brands.
    pub excluded_payment_methods: Vec<ExcludedPaymentMethod>,

    /// Method that excludes undesired payment methods for your operation, such as credit card,
    /// ticket, among others. Note that these are PAYMENT TYPES.
    pub excluded_payment_types: Vec<ExcludedPaymentType>,

    /// Method that defines the maximum number of installments to be offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExcludedPaymentMethod {
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExcludedPaymentType {
    pub id: Option<String>,
}

/// An item processed by MercadoPago.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,

    /// Item ID/sku, identified in your platform.
    pub id: Option<String>,
    pub currency_id: Option<CurrencyId>,
    pub picture_url: Option<String>,
    pub category_id: Option<String>,
}

impl Item {
    pub fn minimal_item(
        name: String,
        description: String,
        price: f64,
        quantity: i32,
    ) -> Result<Item, SDKError> {
        if quantity < 1 {
            return Err(ValidationError::ItemError(
                "You can't have zero of something.".to_string(),
            )
            .into());
        }

        Ok(Self {
            title: name,
            description,
            quantity,
            unit_price: price,

            id: None,
            currency_id: None,
            picture_url: None,
            category_id: None,
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shipments {
    pub receiver_address: Option<Address>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub zip_code: Option<String>,
    pub state_name: Option<String>,
    pub city_name: Option<String>,
    pub street_name: Option<String>,
    pub street_number: Option<i64>,
}

/// A payer will ALWAYS have a `PersonalIdentification`, and an `email` since it's the bare minimum.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckoutProPayer {
    pub(crate) email: Option<String>,
    pub identification: PersonalIdentification,

    pub name: Option<String>,
    pub surname: Option<String>,
    pub phone: Option<Phone>,
    pub address: Option<Address>,
}

impl CheckoutProPayer {
    pub fn validate(&self) -> bool {
        if self.email.is_none()
            || self.identification.number.is_none()
            || self.identification.document_type.is_none()
        {
            return false;
        }
        true
    }

    pub fn standard_payer<II>(
        email: String,
        document_type: DocumentType,
        document_number: II,
    ) -> Self
    where
        II: Into<Option<i64>>,
    {
        Self {
            email: Some(email),
            identification: PersonalIdentification {
                document_type: Some(document_type),
                number: document_number.into(),
            },

            name: None,
            surname: None,
            phone: None,
            address: None,
        }
    }

    pub fn minimal_payer<II>(
        email: String,
        document_type: DocumentType,
        document_number: II,
    ) -> Self
    where
        II: Into<Option<i64>>,
    {
        Self {
            email: Some(email),
            identification: PersonalIdentification {
                document_type: Some(document_type),
                number: document_number.into(),
            },

            name: None,
            surname: None,
            phone: None,
            address: None,
        }
    }
}

/// Documents for personal identification, such as RG, CPF, CNH
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalIdentification {
    #[serde(rename = "type")]
    pub document_type: Option<DocumentType>,
    #[serde(
        default,
        serialize_with = "option_stringify",
        deserialize_with = "serde_aux::field_attributes::deserialize_option_number_from_string"
    )]
    pub number: Option<i64>,
}

impl PersonalIdentification {
    pub fn new(document_type: DocumentType, document_number: i64) -> Self {
        Self {
            document_type: Some(document_type),
            number: Some(document_number),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub first_six_digits: String,
    pub last_four_digits: String,
    pub expiration_month: i64,
    pub expiration_year: i64,

    pub card_number_length: i64,
    pub security_code_length: i64,

    pub cardholder: Cardholder,

    #[serde(with = "time::serde::rfc3339")]
    pub date_created: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_last_updated: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_due: time::OffsetDateTime,
}

/// Information of the Credit/Debit Card owner.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cardholder {
    pub name: String,
    pub identification: PersonalIdentification,
}
