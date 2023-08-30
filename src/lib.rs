//! An open source, strongly-typed SDK for the MercadoPago API.
//!
//! It will try to hold your hand and reduce the possibility of errors, providing the correct API
//! surface.
//!
//! ### Note
//!
//! The library is still under development and its public API is subject to change.
//!
//! # Installation
//!
//! Added the following into your Cargo.toml:
//!
//! ```toml
//! mercadopago_sdk_rust = "0.1"
//! ```
//!
//! # Usage
//!
//! The client is built using the
//! [`MercadoPagoSDKBuilder::with_token`](crate::MercadoPagoSDKBuilder) `with_token`
//! method.
//!
//! ```rust
//! # fn main() {
//! use mercadopago_sdk_rust::{MercadoPagoSDK, MercadoPagoSDKBuilder};
//!
//! let mp_sdk: MercadoPagoSDK = MercadoPagoSDKBuilder::with_token("MP_ACCESS_TOKEN");
//!
//! # }
//! ```
//!
//! Once the token is inserted, you can call methods on [`crate::MercadoPagoSDK`]
//!
//!
//!
//! # Creating a CheckoutPro Preference
//! ```no_run
//! use mercadopago_sdk_rust::common_types::{CheckoutProPayer, Item};
//! use mercadopago_sdk_rust::payments::requests::DocumentType;
//! use mercadopago_sdk_rust::preferences::requests::CheckoutProPreferences;
//! use mercadopago_sdk_rust::MercadoPagoSDKBuilder;
//!
//! #[tokio::main]
//! async fn async_main() {
//!     let mp_sdk = MercadoPagoSDKBuilder::with_token("MP_ACCESS_TOKEN");
//!
//!     let sample_item =
//!         Item::minimal_item("Sample item".to_string(), "".to_string(), 15.00, 1).unwrap();
//!
//!     let preferences = CheckoutProPreferences::new()
//!         .set_items(vec![sample_item])
//!         .set_payer(CheckoutProPayer::minimal_payer(
//!             "fulano@beltrano.com.br".to_string(),
//!             DocumentType::CPF,
//!             41810524485,
//!         ));
//!
//!     mp_sdk
//!         .create_preferences_checkout_pro(preferences)
//!         .expect("Failed to validate checkout preference. Something is wrong.")
//!         .execute()
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! # Other Examples
//!
//! Check out the `tests` folder inside our repository to check for more examples.
//!
//! # License
//! Project is licensed under the permissive MIT license.

pub mod card_tokens;
pub mod common_types;
pub mod errors;
pub mod helpers;
pub mod payments;
pub mod preferences;
pub mod webhooks;

use std::marker::PhantomData;

use futures::future::err;
use futures::TryFutureExt;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AccessToken, AuthType, AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl,
};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::card_tokens::requests::CardTokenOptions;
use crate::card_tokens::responses::CardTokenResponse;
use crate::errors::{ApiError, SDKError};
use crate::payments::requests::CreatePaymentPayload;
use crate::preferences::requests::CheckoutProPreferences;
use crate::preferences::responses::CheckoutProPreferencesResponse;

const API_BASE_URL: &str = "https://api.mercadopago.com";

///
#[derive(Debug)]
pub struct MercadoPagoSDKBuilder {}

impl MercadoPagoSDKBuilder {
    pub async fn authorize<T: ToString>(
        client_id: T,
        client_secret: T,
    ) -> Result<MercadoPagoSDK, SDKError> {
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new("https://auth.mercadopago.com/authorization".to_string()).unwrap(),
            Some(TokenUrl::new("https://api.mercadopago.com/oauth/token".to_string()).unwrap()),
        )
        .set_auth_type(AuthType::BasicAuth);

        let token_response = client
            .exchange_client_credentials()
            .add_scope(Scope::new("offline_access".to_string()))
            .request_async(async_http_client)
            .await
            .unwrap();

        Ok(MercadoPagoSDK {
            http_client: Default::default(),
            access_token: token_response.access_token().clone(),
        })
    }

    /// Creates an [`MercadoPagoSDK`] ready to request the API.
    pub fn with_token<T: ToString>(client_access_token: T) -> MercadoPagoSDK {
        MercadoPagoSDK {
            http_client: Default::default(),
            access_token: AccessToken::new(client_access_token.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct MercadoPagoSDK {
    pub(crate) http_client: Client,
    pub(crate) access_token: AccessToken,
}

#[derive(Debug)]
pub struct SDKRequest<'a, RP> {
    http_client: &'a Client,
    access_token: &'a AccessToken,
    request: RequestBuilder,
    response_type: PhantomData<RP>,
}

impl<'a, RP> SDKRequest<'a, RP> {
    /// Injects bearer token, and return response
    pub async fn execute(self) -> Result<RP, SDKError>
    where
        RP: DeserializeOwned,
    {
        let request = self
            .request
            .bearer_auth(self.access_token.secret())
            .build()
            .unwrap();
        let response = self
            .http_client
            .execute(request)
            .and_then(|c| c.text())
            .await?;
        eprintln!("response = {}", response);

        // matches errors due to wrong payloads etc
        let error_jd = serde_json::from_str::<ApiError>(&*response);
        if let Ok(err) = error_jd {
            eprintln!("err = {:#?}", err);
            return Err(SDKError::GenericError);
        }

        let jd = &mut serde_json::Deserializer::from_str(&*response);
        let res: Result<RP, _> = serde_path_to_error::deserialize(jd);

        match res {
            Ok(deserialized_resp) => Ok(deserialized_resp),
            Err(wow) => {
                println!("{:?}", wow.path());
                eprintln!("Error = {:#?}", wow);
                Err(SDKError::GenericError)
            }
        }
    }
}

impl MercadoPagoSDK {
    pub fn create_preferences_checkout_pro(
        &self,
        opts: CheckoutProPreferences,
    ) -> Result<SDKRequest<CheckoutProPreferencesResponse>, SDKError> {
        if opts.validate() {}

        let request = self
            .http_client
            .request(
                Method::POST,
                API_BASE_URL.to_string() + "/checkout/preferences",
            )
            .json(&opts);

        Ok(SDKRequest {
            http_client: &self.http_client,
            access_token: &self.access_token,
            request,
            response_type: PhantomData::<_>,
        })
    }

    /// Used to create and save a credit/debit card token, instead of transacting raw sensitive
    /// data, such as card number.
    ///
    /// Create a token before issuing payments with cards.
    pub fn create_card_token(
        &self,
        opts: CardTokenOptions,
    ) -> Result<SDKRequest<CardTokenResponse>, SDKError> {
        let url = format!(
            "{}/v1/card_tokens?public_key={}",
            API_BASE_URL,
            opts.public_key.as_deref().unwrap_or("")
        );

        let request = self.http_client.request(Method::POST, url).json(&opts);

        Ok(SDKRequest {
            http_client: &self.http_client,
            access_token: &self.access_token,
            request,
            response_type: PhantomData::<_>,
        })
    }

    pub fn create_payment(
        &self,
        opts: CreatePaymentPayload,
    ) -> Result<SDKRequest<CheckoutProPreferencesResponse>, SDKError> {
        let request = self
            .http_client
            .request(Method::POST, "/payments")
            .json(&opts);

        Ok(SDKRequest {
            http_client: &self.http_client,
            access_token: &self.access_token,
            request,
            response_type: PhantomData::<CheckoutProPreferencesResponse>,
        })
    }
}
