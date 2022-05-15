use serde::{Deserialize, Serialize};

use crate::common_types::Card;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardTokenResponse {
    #[serde(rename = "id")]
    pub card_token_id: String,

    #[serde(flatten)]
    pub card: Card,

    /// Public key that was used to create this token.
    /// Only populated if `public_key` was passed to create a token first.
    pub public_key: Option<String>,

    pub live_mode: bool,
    pub luhn_validation: bool,
    pub require_esc: bool,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_token_response() {
        let payload = serde_json::from_slice::<CardTokenResponse>(include_bytes!(
            "../../tests/assets/card_tokens_create_response.json"
        ));
        assert!(payload.is_ok());
    }
}
