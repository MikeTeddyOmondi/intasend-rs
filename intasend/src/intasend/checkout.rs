#![allow(unused)]
#![allow(unused_imports)]

#[cfg(feature = "client")]
use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{Currency, RequestClient, RequestMethods, Tarrif};

/// `Checkout` struct implements methods for facilitating:
/// Checkout Link API that allows you to generate a secure link that you can
/// share with your customer to complete payment. Once the customer click on the link,
/// IntaSend provides a guide on how to complete payment and notify both you and the customers on the status of the payment.
///
/// ```rust
/// // Load .env file
/// dotenvy::dotenv().ok();
///
/// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
/// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
///
/// // Intasend Client
/// let intasend = intasend::Intasend::new(
///    intasend_public_key,
///    intasend_secret_key,
///     true,
/// );
///
/// // CheckoutsAPI
/// let checkout: intasend::CheckoutsAPI = intasend.checkout();
/// ```
///
#[derive(Debug)]
pub struct CheckoutsAPI {
    pub(crate) intasend: Intasend,
}

impl CheckoutsAPI {
    /// The `initiate` method generates a checkout link that you'll send to your customers to complete payment
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// // Load .env file
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    ///
    /// // CheckoutsAPI
    /// let checkout: intasend::CheckoutsAPI = intasend.checkout();
    ///
    /// let checkout_req = intasend::CheckoutRequest {
    ///     first_name: Some("Joe".to_string()),
    ///     last_name: Some("Doe".to_string()),
    ///     email: Some("joe@doe.com".to_string()),
    ///     method: Some(intasend::CheckoutMethod::CardPayment),
    ///     amount: rust_decimal::Decimal::new(10, 2),
    ///     currency: intasend::Currency::Usd,
    /// };
    ///
    /// let checkout_response: intasend::CheckoutResponse = checkout.initiate(checkout_req).await?;
    /// println!("[#] Checkout Init: {:?}", checkout_response);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn initiate(&self, payload: CheckoutRequest) -> Result<CheckoutResponse, Error> {
        let service_path: &str = "/api/v1/checkout/";
        let request_method: RequestMethods = RequestMethods::Post;

        let checkout_response = &self
            .intasend
            .send_client_request::<CheckoutRequest, CheckoutResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;
        // println!("[#] Checkout Response: {:#?}", checkout_response);

        Ok(checkout_response.clone())
    }

    /// The `details` method approves send money request.
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// // Load .env file
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // CheckoutsAPI
    /// let checkout: intasend::CheckoutsAPI = intasend.checkout();
    ///
    /// let checkout_details_req = intasend::CheckoutDetailsRequest {
    ///     checkout_id: "dd4bd8a2-a34c-4c04-9663-6935eb8a8a4b".to_string(),
    ///     signature: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyJkZDRiZDhhMi1hMzRjLTRjMDQtOTY2My02OTM1ZWI4YThhNGIiXSwiaWF0IjoxNzA4NTQ1ODgyLCJleHAiOjE3MDg1NDk0ODIsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiJkZDRiZDhhMi1hMzRjLTRjMDQtOTY2My02OTM1ZWI4YThhNGIifQ.lnHBsBqzAaM24UxdL82nrGIlpDKBlGG-tCJDocMkrZk".to_string(),
    /// };
    ///
    /// let checkout_details_response: intasend::CheckoutDetailsResponse = checkout.details(checkout_details_req).await?;
    /// println!("Checkout details response: {:#?}", checkout_details_response);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn details(
        &self,
        payload: CheckoutDetailsRequest,
    ) -> Result<CheckoutDetailsResponse, Error> {
        let service_path: &str = "/api/v1/checkout/details/";
        let request_method: RequestMethods = RequestMethods::Post;

        let checkout_details_response = &self
            .intasend
            .send_client_request::<CheckoutDetailsRequest, CheckoutDetailsResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;
        // println!("[#] Checkout Details Response: {:#?}", checkout_details_response);

        Ok(checkout_details_response.clone())
    }
}

/// `CheckoutRequest` Struct - `CheckoutsAPI`
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub method: Option<CheckoutMethod>,
    pub amount: Decimal,
    pub currency: Currency,
}

/// `CheckoutResponse` Struct - `CheckoutsAPI`
///
/// **Note**: persist the `id` and the `signature` field in a store if you want to get the details of the fund checkout
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckoutResponse {
    pub id: String,
    pub url: String,
    pub signature: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub method: Option<CheckoutMethod>,
    pub amount: Decimal,
    pub currency: Currency,
    pub paid: bool,
}

/// `CheckoutDetailsRequest` Struct - `Checkout` API
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutDetailsRequest {
    pub checkout_id: String,
    pub signature: String,
}

/// `CheckoutDetailsResponse` Struct - `Checkout` API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckoutDetailsResponse {
    pub id: String,
    pub url: String,
    pub signature: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub api_ref: Option<String>,
    pub wallet_id: Option<String>,
    pub method: Option<CheckoutMethod>,
    pub channel: Option<String>,
    pub host: Option<String>,
    pub is_mobile: bool,
    pub version: Option<String>,
    pub redirect_url: Option<String>,
    pub amount: Decimal,
    pub currency: Option<Currency>,
    pub paid: bool,
    pub mobile_tarrif: Tarrif,
    pub card_tarrif: Tarrif,
    pub bitcoin_tarrif: Tarrif,
    pub ach_tarrif: Tarrif,
    pub created_at: String,
    pub updated_at: String,
    pub defaults: CheckoutDefaults,
}

/// `Checkout` Options supported by `Intasend` API Gateway
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CheckoutMethod {
    Mpesa,
    #[serde(rename = "CARD-PAYMENT")]
    CardPayment,
    Bitcoin,
    #[serde(rename = "BANK-ACH")]
    Bank,
    #[serde(rename = "COOP_B2B")]
    CoopB2b,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckoutDefaults {
    pub enable_card_payment: bool,
    pub enable_mpesa_payment: bool,
    pub enable_bitcoin_payment: bool,
    pub enable_ach_payment: bool,
    pub default_currency: Currency,
    pub default_tarrif: Tarrif,
}
