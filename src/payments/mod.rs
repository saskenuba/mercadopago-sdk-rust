//! When creating payment for CreditCard, use `card_tokens` to generate a credit card token.
//!
//!
//! You will be always handling tokens instead of the raw credit/debit card information. You can
//! store it on your database if you want to.

pub mod requests;
pub mod responses;
