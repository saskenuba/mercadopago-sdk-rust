use serde::{Deserialize, Serialize};

use crate::common_types::Cardholder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardTokenPayload {
    pub card_number: String,
    pub expiration_month: String,
    pub expiration_year: String,
    pub security_code: String,
    pub cardholder: Cardholder,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_token_serialization() {
        #[test]
        fn t_token_response() {
            let payload = serde_json::from_slice::<CardTokenPayload>(include_bytes!(
                "../../tests/assets/card_tokens_create_request.json"
            ))
            .unwrap();
            println!("{:?}", payload);
        }
    }
}
