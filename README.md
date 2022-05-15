# MercadoPago SDK


[![Crate version on crates.io](https://img.shields.io/crates/v/mercadopago-sdk-rust)](https://crates.io/crates/mercadopago-sdk-rust)
[![Crate documentation on docs.rs](https://img.shields.io/docsrs/mercadopago-sdk-rust)](https://docs.rs/mercadopago-sdk-rust)
![Crate license](https://img.shields.io/crates/l/mercadopago-sdk-rust)


<!-- cargo-rdme start -->

An open source, strongly-typed SDK for the MercadoPago API.

It will try to hold your hand and reduce the possibility of errors, providing the correct API
surface.

### Note

The library is still under development and its public API is subject to change.

# Installation

Added the following into your Cargo.toml:

```toml
mercadopago_sdk_rust = "0.1"
```

# Usage

The client is built using the
[`MercadoPagoSDKBuilder::with_token`](https://docs.rs/mercadopago-sdk-rust/latest/mercadopago_sdk_rust/struct.MercadoPagoSDKBuilder.html) `with_token`
method.

```rust
use mercadopago_sdk_rust::{MercadoPagoSDK, MercadoPagoSDKBuilder};

let mp_sdk: MercadoPagoSDK = MercadoPagoSDKBuilder::with_token("MP_ACCESS_TOKEN");

```

Once the token is inserted, you can call methods on [`crate::MercadoPagoSDK`]



# Creating a CheckoutPro Preference
```rust
use mercadopago_sdk_rust::common_types::{CheckoutProPayer, Item};
use mercadopago_sdk_rust::payments::requests::DocumentType;
use mercadopago_sdk_rust::preferences::requests::CheckoutProPreferences;
use mercadopago_sdk_rust::MercadoPagoSDKBuilder;

#[tokio::main]
async fn async_main() {
    let mp_sdk = MercadoPagoSDKBuilder::with_token("MP_ACCESS_TOKEN");

    let sample_item =
        Item::minimal_item("Sample item".to_string(), "".to_string(), 15.00, 1).unwrap();

    let preferences = CheckoutProPreferences::new()
        .set_items(vec![sample_item])
        .set_payer(CheckoutProPayer::minimal_payer(
            "fulano@beltrano.com.br".to_string(),
            DocumentType::CPF,
            41810524485,
        ));

    mp_sdk
        .create_preferences_checkout_pro(preferences)
        .expect("Failed to validate checkout preference. Something is wrong.")
        .execute()
        .await
        .unwrap();
}
```

# Other Examples

Check out the `tests` folder inside our repository to check for more examples.

# License
Project is licensed under the permissive MIT license.

<!-- cargo-rdme end -->
