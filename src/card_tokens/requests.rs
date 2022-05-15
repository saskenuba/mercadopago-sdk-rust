use serde::{Deserialize, Serialize};

use crate::common_types::{Cardholder, PersonalIdentification};

/// Options used to create a Card Token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardTokenOptions<'a> {
    pub(crate) card_number: &'a str,
    pub(crate) expiration_month: &'a str,
    pub(crate) expiration_year: &'a str,
    pub(crate) security_code: &'a str,
    pub(crate) cardholder: Cardholder,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) public_key: Option<String>,
}

impl<'a> CardTokenOptions<'a> {
    pub fn new(
        card_number: &'a str,
        card_expiration_month: &'a str,
        card_expiration_year: &'a str,
        security_code: &'a str,
        cardholder_name: String,
        cardholder_personal_id: PersonalIdentification,
    ) -> CardTokenOptions<'a> {
        Self {
            card_number,
            expiration_month: card_expiration_month,
            expiration_year: card_expiration_year,
            security_code,
            cardholder: Cardholder {
                name: cardholder_name,
                identification: cardholder_personal_id,
            },

            public_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_token_serialization() {
        #[test]
        fn t_token_response() {
            let payload = serde_json::from_slice::<CardTokenOptions>(include_bytes!(
                "../../tests/assets/card_tokens_create_request.json"
            ))
            .unwrap();
            println!("{:?}", payload);
        }
    }
}
