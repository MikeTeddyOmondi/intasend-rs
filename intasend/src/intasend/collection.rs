use std::net::Ipv4Addr;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::Intasend;

use super::{RequestClient, RequestMethods};

#[derive(Debug)]
pub struct Collection {
    pub(crate) intasend: Intasend,
}

impl Collection {
    // pub async fn charge<'a>(&self, payload: ChargeRequest) -> Result<ChargeResponse, Error> {
    //     let client = Client::new();

    //     let base_url = if self.intasend.test_mode {
    //         "https://sandbox.intasend.com"
    //     } else {
    //         "https://payment.intasend.com"
    //     };

    //     // TODO!:
    //     let service_path: &str = "/api/v1/checkout/";
    //     let request_method: RequestMethods = RequestMethods::POST;

    //     // self.intasend.send::<'a>(payload, service_path, request_method);
    //     let charge_response = <Intasend as RequestClient<ChargeRequest, ChargeResponse>>::send(
    //         &self.intasend,
    //         payload,
    //         service_path,
    //         request_method,
    //     )
    //     .await?;

    //     // let response = client
    //     //     .post(&format!("{}/api/v1/checkout/", base_url))
    //     //     .header("Content-Type", "application/json")
    //     //     .header(
    //     //         "Authorization",
    //     //         format!("Bearer {}", self.intasend.secret_key),
    //     //     )
    //     //     .header(
    //     //         "INTASEND_PUBLIC_API_KEY",
    //     //         self.intasend.publishable_key.clone(),
    //     //     )
    //     //     .json(&payload)
    //     //     .send()
    //     //     .await;

    //     // let charge_response: ChargeResponse = response?.json().await?;

    //     Ok(charge_response)
    // }

    pub async fn mpesa_stk_push(
        &self,
        payload: MpesaStkPushRequest,
    ) -> Result<MpesaStkPushResponse, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        // TODO!:
        let service_path: &str = "/api/v1/payment/mpesa-stk-push/";
        let request_method: RequestMethods = RequestMethods::POST;

        let mpesa_stk_push_response = <Intasend as RequestClient<
            MpesaStkPushRequest
        >>::send(
            &self.intasend, payload, service_path, request_method
        )
        .await?;

        // let response = client
        //     .post(&format!("{}/api/v1/payment/mpesa-stk-push/", base_url))
        //     .header("Content-Type", "application/json")
        //     .header(
        //         "Authorization",
        //         format!("Bearer {}", self.intasend.secret_key),
        //     )
        //     .header(
        //         "INTASEND_PUBLIC_API_KEY",
        //         self.intasend.publishable_key.clone(),
        //     )
        //     .json(&payload)
        //     .send()
        //     .await;

        // let mpesa_stk_push_response: MpesaStkPushResponse = response?.json().await?;

        Ok(mpesa_stk_push_response)
    }

    pub async fn status(&self, payload: StatusRequest) -> Result<StatusResponse, Error> {
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

        let status_response: StatusResponse = response?.json().await?;

        Ok(status_response)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChargeRequest {
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChargeResponse {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MpesaStkPushRequest {
    pub amount: u32,
    pub phone_number: String,
    pub api_ref: Option<String>,
    pub wallet_id: Option<String>,
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct MpesaStkPushResponse {
//     pub id: String,
//     pub amount: u32,
//     pub currency: String,
//     pub recipient: String,
//     pub method: String,
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusRequest {
    pub invoice_id: String,
    pub signature: Option<String>,
    pub checkout_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusResponse {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub struct MpesaStkPushResponse {
    pub invoice: Option<Invoice>,
    pub customer: Option<Customer>,
    pub payment_link: Option<String>,
    pub refundable: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Invoice {
    invoice_id: String,
    state: String,
    provider: String,
    charges: f64,
    net_amount: f64,
    currency: String,
    value: f64,
    account: String,
    api_ref: String,
    mpesa_reference: Option<String>,
    host: f64,
    card_info: CardInfo,
    retry_count: u32,
    failed_reason: Option<String>,
    failed_code: Option<u32>,
    failed_code_link: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct Customer {
    customer_id: String,
    phone_number: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    country: Option<String>,
    zipcode: Option<String>,
    provider: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CardInfo {
    bin_country: Option<String>,
    card_type: Option<String>,
}