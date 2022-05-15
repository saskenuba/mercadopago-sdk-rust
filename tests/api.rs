use mercadopago_sdk_rust::common_types::{Item, PreferencePayerInformation};
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
        .set_payer(PreferencePayerInformation::minimal_payer(
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
