#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{Currency, FromJsonValue, RequestClient, RequestMethods};

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
/// // Collection API
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
        println!("Response: {:#?}", json_response);

        let checkout_response = CheckoutResponse::from_value(&json_response).unwrap();
        println!("Response: {:#?}", checkout_response);

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
    /// let checkout_details_response: CheckoutResponse = checkout.details(checkout_details_req).await?;
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
            .header(
                "Authorization",
                format!("Bearer {}", self.intasend.secret_key),
            )
            .header(
                "INTASEND_PUBLIC_API_KEY",
                self.intasend.publishable_key.clone(),
            )
            .json(&payload)
            .send()
            .await;

        let status_response: CheckoutDetailsResponse = response?.json().await?;

        Ok(status_response)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub method: Option<String>,
    pub amount: Decimal,
    pub currency: Currency,
}

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
        // let id = value
        //     .get("id")
        //     .unwrap()
        //     .as_str()
        //     .ok_or(Error::msg("id field at not found"))?
        //     .to_string();

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutDetailsRequest {
    pub checkout_id: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutDetailsResponse {
    pub invoice: Option<String>,
    pub customer: Option<String>,
    pub payment_link: Option<String>,
    pub refundable: bool,
    pub created_at: String,
    pub updated_at: String,
}

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

// Checkout Response
// {
//   "id": "0d035afb-8a8b-49ba-ac75-e3c756d8528b",
//   "url": "https://sandbox.intasend.com/checkout/0d035afb-8a8b-49ba-ac75-e3c756d8528b/express/",
//   "signature": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyIwZDAzNWFmYi04YThiLTQ5YmEtYWM3NS1lM2M3NTZkODUyOGIiXSwiaWF0IjoxNzA3ODQxMjU1LCJleHAiOjE3MDc4NDQ4NTUsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiIwZDAzNWFmYi04YThiLTQ5YmEtYWM3NS1lM2M3NTZkODUyOGIifQ.hCpvhRGxwMZqoNK7jXiMk-7BHFhPeWVM_fQQan74hU8",
//   "first_name": null,
//   "last_name": null,
//   "phone_number": null,
//   "email": null,
//   "country": null,
//   "address": null,
//   "city": null,
//   "state": null,
//   "zipcode": null,
//   "api_ref": null,
//   "wallet_id": null,
//   "method": null,
//   "channel": "WEBSITE",
//   "host": "127.0.0.1",
//   "is_mobile": false,
//   "version": null,
//   "redirect_url": null,
//   "amount": "0.00",
//   "currency": "KES",
//   "paid": false,
//   "mobile_tarrif": "BUSINESS-PAYS",
//   "card_tarrif": "BUSINESS-PAYS",
//   "bitcoin_tarrif": "BUSINESS-PAYS",
//   "ach_tarrif": "BUSINESS-PAYS",
//   "created_at": "2024-02-13T19:20:55.215052+03:00",
//   "updated_at": "2024-02-13T19:20:55.215391+03:00"
// }

//   "id": "7e8e822a-0c26-44f7-8fda-060f806ef1e0",
//   "url": "https://sandbox.intasend.com/checkout/7e8e822a-0c26-44f7-8fda-060f806ef1e0/express/",
//   "signature": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyI3ZThlODIyYS0wYzI2LTQ0ZjctOGZkYS0wNjBmODA2ZWYxZTAiXSwiaWF0IjoxNzA3OTA0OTk0LCJleHAiOjE3MDc5MDg1OTQsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiI3ZThlODIyYS0wYzI2LTQ0ZjctOGZkYS0wNjBmODA2ZWYxZTAifQ.RgvVSeQ3mGlEOAabjc74pMtUzpR6HEuyGW7WOBVzVeA",
//   "first_name": null,
//   "last_name": null,
//   "phone_number": null,
//   "email": null,

// Checkout Details Response
// {
//   "id": "dd4bd8a2-a34c-4c04-9663-6935eb8a8a4b",
//   "url": "https://sandbox.intasend.com/checkout/dd4bd8a2-a34c-4c04-9663-6935eb8a8a4b/express/",
//   "signature": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyJkZDRiZDhhMi1hMzRjLTRjMDQtOTY2My02OTM1ZWI4YThhNGIiXSwiaWF0IjoxNzA4NTQ2MTI4LCJleHAiOjE3MDg1NDk3MjgsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiJkZDRiZDhhMi1hMzRjLTRjMDQtOTY2My02OTM1ZWI4YThhNGIifQ.Pa6EwuVJhV5LtLU_NkIprarxkHgoywjlV_CK7L68uj4",
//   "first_name": null,
//   "last_name": null,
//   "phone_number": null,
//   "email": null,
//   "country": null,
//   "address": null,
//   "city": null,
//   "state": null,
//   "zipcode": null,
//   "api_ref": null,
//   "wallet_id": null,
//   "method": null,
//   "channel": "WEBSITE",
//   "host": "127.0.0.1",
//   "is_mobile": false,
//   "version": null,
//   "redirect_url": null,
//   "amount": "0.00",
//   "currency": "KES",
//   "paid": false,
//   "mobile_tarrif": "BUSINESS-PAYS",
//   "card_tarrif": "BUSINESS-PAYS",
//   "bitcoin_tarrif": "BUSINESS-PAYS",
//   "ach_tarrif": "BUSINESS-PAYS",
//   "created_at": "2024-02-21T23:04:42.566094+03:00",
//   "updated_at": "2024-02-21T23:04:42.566121+03:00",
//   "defaults": {
//     "enable_card_payment": true,
//     "enable_mpesa_payment": true,
//     "enable_bitcoin_payment": false,
//     "enable_ach_payment": false,
//     "default_currency": "USD",
//     "default_tarrif": "BUSINESS-PAYS"
//   }
// }