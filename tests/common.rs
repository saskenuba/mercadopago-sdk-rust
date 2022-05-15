/// ! Access token is the same used on official MercadoPago SDKs.
use mercadopago_sdk_rust::{MercadoPagoSDK, MercadoPagoSDKBuilder};

pub fn create_sdk() -> MercadoPagoSDK {
    MercadoPagoSDKBuilder::with_token(
        "APP_USR-558881221729581-091712-44fdc612e60e3e638775d8b4003edd51-471763966",
    )
}
