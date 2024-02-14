#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{Currency, FromJsonValue, RequestClient, RequestMethods};

#[derive(Debug)]
pub struct Checkout {
    pub(crate) intasend: Intasend,
}

impl Checkout {
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
            .post(&format!("{}/api/v1/payment/status/", base_url))
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

        let status_response = CheckoutResponse::from_value(&json_response).unwrap();
        println!("Response: {:#?}", status_response);

        // let status_response: CheckoutResponse = response?.json().await?;

        Ok(status_response)
    }

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
            .post(&format!("{}/api/v1/payment/status/", base_url))
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
    pub method: Option<CheckoutMethod>,
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
    pub invoice: Option<String>,
    pub customer: Option<String>,
    pub payment_link: Option<String>,
    pub refundable: bool,
    pub created_at: String,
    pub updated_at: String,
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
    fn as_str(&self) -> String {
        match self {
            CheckoutMethod::MPESA => "M-PESA".to_string(),
            CheckoutMethod::CARDPAYMENT => "CARD-PAYMENT".to_string(),
            CheckoutMethod::BITCOIN => "BITCOIN".to_string(),
            CheckoutMethod::BANKACH => "BANK-ACH".to_string(),
            CheckoutMethod::COOPB2B => "COOP_B2B".to_string(),
        }
    }
}

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
