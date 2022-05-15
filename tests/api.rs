use mercadopago_sdk_rust::card_tokens::requests::CardTokenOptions;
use mercadopago_sdk_rust::common_types::{CheckoutProPayer, Item, PersonalIdentification};
use mercadopago_sdk_rust::payments::requests::DocumentType;
use mercadopago_sdk_rust::preferences::requests::CheckoutProPreferences;

mod common;

#[tokio::test]
async fn create_preference() {
    let sdk = common::create_sdk();

    let sample_item =
        Item::minimal_item("Sample item".to_string(), "".to_string(), 15.00, 1).unwrap();

    let preferences = CheckoutProPreferences::new()
        .set_items(vec![sample_item])
        .set_payer(CheckoutProPayer::minimal_payer(
            "fulano@beltrano.com.br".to_string(),
            DocumentType::CPF,
            41810524485,
        ));

    sdk.create_preferences_checkout_pro(preferences)
        .expect("TODO: panic message")
        .execute()
        .await
        .unwrap();
}

#[tokio::test]
async fn tokenize_card() {
    let sdk = common::create_sdk();

    let card_owner_pi = PersonalIdentification::new(DocumentType::CPF, 12345678909);
    let token_opts = CardTokenOptions::new(
        "5031433215406351",
        "11",
        "2025",
        "123",
        "APRO".to_string(),
        card_owner_pi,
    );

    sdk.create_card_token(token_opts)
        .expect("TODO: panic message")
        .execute()
        .await
        .unwrap();
}
