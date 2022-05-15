extern crate core;

/// A open source SDK for the MercadoPago API.
pub mod card_tokens;
pub mod common_types;
pub mod errors;
pub mod helpers;
pub mod payments;
pub mod preferences;
pub mod webhooks;

use std::marker::PhantomData;

use futures::{FutureExt, TryFutureExt};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AccessToken, AuthType, AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl,
};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::errors::SDKError;
use crate::payments::requests::CreatePaymentPayload;
use crate::preferences::requests::CheckoutProPreferences;
use crate::preferences::responses::CheckoutProPreferencesResponse;

const CLIENT_ID: &str = "1673647513457968";
const API_BASE_URL: &str = "https://api.mercadopago.com";

#[derive(Debug)]
pub struct MercadoPagoSDKBuilder {}

impl MercadoPagoSDKBuilder {
    async fn authorize<T: ToString>(
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

        let jd = &mut serde_json::Deserializer::from_str(&*response);
        let res: Result<RP, _> = serde_path_to_error::deserialize(jd);

        let oi = match res {
            Ok(a) => a,
            Err(wow) => {
                println!("{:?}", wow.path());
                eprintln!("Error = {:#?}", wow);
                return Err(SDKError::GenericError);
            }
        };

        Ok(oi)
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
            response_type: PhantomData::<CheckoutProPreferencesResponse>,
        })
    }
    /// Used to create and save a credit/debit card token, instead of transacting raw sensitive
    /// data, such as card number.
    ///
    /// Create a token before issuing payments with cards.
    pub fn create_card_token(
        &self,
    ) -> Result<SDKRequest<CheckoutProPreferencesResponse>, SDKError> {
        let request = self
            .http_client
            .request(Method::POST, API_BASE_URL.to_string() + "/card_tokens")
            .json(&None::<&str>);

        Ok(SDKRequest {
            http_client: &self.http_client,
            access_token: &self.access_token,
            request,
            response_type: PhantomData::<CheckoutProPreferencesResponse>,
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
