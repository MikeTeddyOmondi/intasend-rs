#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{Currency, FromJsonValue, RequestClient, RequestMethods, Tarrif};

/// Checkout struct implements methods for facilitating:
/// Checkout Link API that allows you to generate a secure link that you can
/// share with your customer to complete payment. Once the customer click on the link,
/// IntaSend provides a guide on how to complete payment and notify both you and the customers on the status of the payment.
///
/// ```rust
/// // Intasend client
/// let intasend = Intasend::new(
///     publishable_key: "ISPubKey_test_c1f90113-3dbb-4201-9b88-f1c2d3056e5c".to_string(),
///     secret_key: "ISSecretKey_test_5527b085-40b6-460a-9c31-25d58a204d20".to_string(),
///     test_mode: true
/// );
///
/// // Checkout API
/// let checkout: Checkout = intasend.checkout();
/// ```
///
#[derive(Debug)]
pub struct Checkout {
    pub(crate) intasend: Intasend,
}

impl Checkout {
    /// The initiate method generates a checkout link that you'll send to your customers to complete payment
    ///
    /// ```rust
    /// // Checkout API
    /// let checkout: Checkout = intasend.checkout();
    ///
    /// let checkout_req = CheckoutRequest {
    ///     first_name: Some("Joe".to_string()),
    ///     last_name: Some("Doe".to_string()),
    ///     email: Some("joe@doe.com".to_string()),
    ///     method: Some(CheckoutMethod::CARDPAYMENT.as_str()),
    ///     amount: Decimal::new(10, 2),
    ///     currency: Currency::USD,
    /// };
    ///
    /// let checkout_response: CheckoutResponse = checkout.initiate(checkout_req).await?;
    /// println!("Checkout response: {:#?}", checkout_response);
    /// ```
    ///
    pub async fn initiate(&self, payload: CheckoutRequest) -> Result<CheckoutResponse, Error> {
        // let service_path: &str = "/api/v1/checkout/";
        // let request_method: RequestMethods = RequestMethods::POST;

        // // self.intasend.send::<'a>(payload, service_path, request_method);
        // let json_response = <Intasend as RequestClient<CheckoutRequest>>::send(
        //     &self.intasend,
        //     payload,
        //     service_path,
        //     request_method,
        // )
        // .await?;
        // println!("Json Response: {:?}", json_response);

        // let charge_response = CheckoutResponse::from_value(&json_response).unwrap();
        // println!("Json Response: {:#?}", charge_response);

        // Ok(charge_response)

        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/checkout/", base_url))
            .header("Content-Type", "application/json")
            // .header(
            //     "Authorization",
            //     format!("Bearer {}", self.intasend.secret_key),
            // )
            .header(
                "X-IntaSend-Public-API-Key",
                self.intasend.publishable_key.clone(),
            )
            .json(&payload)
            .send()
            .await;

        let json_response =
            serde_json::from_value::<JSON>(response?.json().await?).expect("Error parsing json!");
        // println!("Response: {:#?}", json_response);

        let checkout_response = CheckoutResponse::from_value(&json_response).unwrap();
        // println!("Response: {:#?}", checkout_response);

        // let status_response: CheckoutResponse = response?.json().await?;

        Ok(checkout_response)
    }

    /// The details method approves send money request.
    ///
    /// ```rust
    /// // Checkout API
    /// let checkout: Checkout = intasend.checkout();
    ///
    /// let checkout_details_req = CheckoutDetailsRequest {
    ///     checkout_id: "dd4bd8a2-a34c-4c04-9663-6935eb8a8a4b".to_string(),
    ///     signature: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyJkZDRiZDhhMi1hMzRjLTRjMDQtOTY2My02OTM1ZWI4YThhNGIiXSwiaWF0IjoxNzA4NTQ1ODgyLCJleHAiOjE3MDg1NDk0ODIsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiJkZDRiZDhhMi1hMzRjLTRjMDQtOTY2My02OTM1ZWI4YThhNGIifQ.lnHBsBqzAaM24UxdL82nrGIlpDKBlGG-tCJDocMkrZk".to_string(),
    /// };
    ///
    /// let checkout_details_response: CheckoutDetailsResponse = checkout.details(checkout_details_req).await?;
    /// println!("Checkout details response: {:#?}", checkout_details_response);
    /// ```
    ///
    pub async fn details(
        &self,
        payload: CheckoutDetailsRequest,
    ) -> Result<CheckoutDetailsResponse, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/checkout/details/", base_url))
            .header("Content-Type", "application/json")
            // .header(
            //     "Authorization",
            //     format!("Bearer {}", self.intasend.secret_key),
            // )
            .header(
                "X-IntaSend-Public-API-Key",
                self.intasend.publishable_key.clone(),
            )
            .json(&payload)
            .send()
            .await;

        let json_response =
            serde_json::from_value::<JSON>(response?.json().await?).expect("Error parsing json!");
        // println!("Response: {:#?}", json_response);

        let checkout_details_response =
            CheckoutDetailsResponse::from_value(&json_response).unwrap();
        // println!("Response: {:#?}", checkout_details_response);

        Ok(checkout_details_response)
    }
}

/// CheckoutRequest Struct - Checkout API
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub method: Option<String>,
    pub amount: Decimal,
    pub currency: Currency,
}

/// CheckoutResponse Struct - Checkout API
#[derive(Debug, Serialize, Deserialize)]
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

impl FromJsonValue for CheckoutResponse {
    fn from_value(value: &JSON) -> Result<Self, anyhow::Error> {
        let id = serde_json::from_value(value.get("id").unwrap().clone()).unwrap();
        let url = value
            .get("url")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("url field at not found"))?
            .to_string();
        let signature = value
            .get("signature")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("signature field at not found"))?
            .to_string();
        let first_name = serde_json::from_value(value.get("first_name").unwrap().clone()).unwrap();
        let last_name = serde_json::from_value(value.get("last_name").unwrap().clone()).unwrap();
        let email = value.get("email").unwrap().as_str().map(|s| s.to_string());
        let method: Option<CheckoutMethod> =
            serde_json::from_value(value.get("method").unwrap().clone()).unwrap();
        let amount: Decimal = serde_json::from_value(value.get("amount").unwrap().clone()).unwrap();
        let currency: Currency =
            serde_json::from_value(value.get("currency").unwrap().clone()).unwrap();
        let paid = value
            .get("paid")
            .unwrap()
            .as_bool()
            .ok_or(Error::msg("Refundable not found"))?;

        Ok::<CheckoutResponse, Error>(CheckoutResponse {
            id,
            url,
            signature,
            first_name,
            last_name,
            email,
            method,
            amount,
            currency,
            paid,
        })
    }
}

/// CheckoutDetailsRequest Struct - Checkout API
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutDetailsRequest {
    pub checkout_id: String,
    pub signature: String,
}

/// CheckoutDetailsResponse Struct - Checkout API
#[derive(Debug, Serialize, Deserialize)]
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

impl FromJsonValue for CheckoutDetailsResponse {
    fn from_value(value: &JSON) -> Result<Self, anyhow::Error> {
        let id = serde_json::from_value(value.get("id").unwrap().clone()).unwrap();
        let url = value
            .get("url")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("url field at not found"))?
            .to_string();
        let signature = value
            .get("signature")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("signature field at not found"))?
            .to_string();
        let first_name = serde_json::from_value(value.get("first_name").unwrap().clone()).unwrap();
        let last_name = serde_json::from_value(value.get("last_name").unwrap().clone()).unwrap();
        let phone_number =
            serde_json::from_value(value.get("phone_number").unwrap().clone()).unwrap();
        let email = value.get("email").unwrap().as_str().map(|s| s.to_string());
        let country = serde_json::from_value(value.get("country").unwrap().clone()).unwrap();
        let address = serde_json::from_value(value.get("address").unwrap().clone()).unwrap();
        let city = serde_json::from_value(value.get("city").unwrap().clone()).unwrap();
        let state = serde_json::from_value(value.get("state").unwrap().clone()).unwrap();
        let zipcode = serde_json::from_value(value.get("zipcode").unwrap().clone()).unwrap();
        let api_ref = serde_json::from_value(value.get("api_ref").unwrap().clone()).unwrap();
        let wallet_id = serde_json::from_value(value.get("wallet_id").unwrap().clone()).unwrap();
        let method: Option<CheckoutMethod> =
            serde_json::from_value(value.get("method").unwrap().clone()).unwrap();
        let channel = serde_json::from_value(value.get("channel").unwrap().clone()).unwrap();
        let host = serde_json::from_value(value.get("host").unwrap().clone()).unwrap();
        let is_mobile = value
            .get("is_mobile")
            .unwrap()
            .as_bool()
            .ok_or(Error::msg("is_mobile field not found"))?;
        let version = serde_json::from_value(value.get("version").unwrap().clone()).unwrap();
        let redirect_url =
            serde_json::from_value(value.get("redirect_url").unwrap().clone()).unwrap();
        let amount: Decimal = serde_json::from_value(value.get("amount").unwrap().clone()).unwrap();
        let currency: Option<Currency> =
            Some(serde_json::from_value(value.get("currency").unwrap().clone()).unwrap());
        let paid = value
            .get("paid")
            .unwrap()
            .as_bool()
            .ok_or(Error::msg("Refundable not found"))?;
        let mobile_tarrif =
            Tarrif::from_str(serde_json::from_value(value.get("mobile_tarrif").unwrap().clone()).unwrap()).expect("Invalid Tarrif value");
        let card_tarrif = Tarrif::from_str(
            serde_json::from_value(value.get("card_tarrif").unwrap().clone()).expect("Invalid Tarrif value"),
        ).expect("Invalid Tarrif value");
        let bitcoin_tarrif =
            Tarrif::from_str(serde_json::from_value(value.get("bitcoin_tarrif").unwrap().clone()).unwrap()).expect("Invalid Tarrif value"); 
        let ach_tarrif = Tarrif::from_str(serde_json::from_value(value.get("ach_tarrif").unwrap().clone()).unwrap()).expect("Invalid Tarrif value");
        let created_at = value
            .get("created_at")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("created_at field not found"))?
            .to_string();
        let updated_at = value
            .get("updated_at")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("updated_at field not found"))?
            .to_string();
        let defaults: CheckoutDefaults =
            serde_json::from_value(value.get("defaults").unwrap().clone()).unwrap();

        Ok::<CheckoutDetailsResponse, Error>(CheckoutDetailsResponse {
            id,
            url,
            signature,
            first_name,
            last_name,
            email,
            method,
            amount,
            currency,
            paid,
            phone_number,
            country,
            address,
            city,
            state,
            zipcode,
            api_ref,
            wallet_id,
            channel,
            host,
            is_mobile,
            version,
            redirect_url,
            mobile_tarrif,
            card_tarrif,
            bitcoin_tarrif,
            ach_tarrif,
            created_at,
            updated_at,
            defaults,
        })
    }
}

/// Checkout Options supported by Intasend API Gateway 
#[derive(Debug, Serialize, Deserialize)]
pub enum CheckoutMethod {
    MPESA,
    CARDPAYMENT,
    BITCOIN,
    BANKACH,
    COOPB2B,
}

impl CheckoutMethod {
    pub fn as_str(&self) -> String {
        match self {
            CheckoutMethod::MPESA => "M-PESA".to_string(),
            CheckoutMethod::CARDPAYMENT => "CARD-PAYMENT".to_string(),
            CheckoutMethod::BITCOIN => "BITCOIN".to_string(),
            CheckoutMethod::BANKACH => "BANK-ACH".to_string(),
            CheckoutMethod::COOPB2B => "COOP_B2B".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutDefaults {
    pub enable_card_payment: bool,
    pub enable_mpesa_payment: bool,
    pub enable_bitcoin_payment: bool,
    pub enable_ach_payment: bool,
    pub default_currency: Currency,
    pub default_tarrif: String,
}
