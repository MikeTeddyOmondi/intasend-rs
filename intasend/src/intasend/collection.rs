use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::Intasend;

use super::{RequestClient, RequestMethods};

#[derive(Debug)]
pub struct Collection {
    pub(crate) intasend: Intasend,
}

impl Collection {
    pub async fn charge<'a>(&self, payload: ChargeRequest) -> Result<ChargeResponse, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        // TODO!:
        let service_path: &str = "/api/v1/checkout/";
        let request_method: RequestMethods = RequestMethods::POST;

        // self.intasend.send::<'a>(payload, service_path, request_method);
        let charge_response = <Intasend as RequestClient<ChargeRequest, ChargeResponse>>::send(
            &self.intasend,
            payload,
            service_path,
            request_method,
        )
        .await?;

        // let response = client
        //     .post(&format!("{}/api/v1/checkout/", base_url))
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

        // let charge_response: ChargeResponse = response?.json().await?;

        Ok(charge_response)
    }

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

        let mpesa_stk_push_response = <Intasend as RequestClient<MpesaStkPushRequest, MpesaStkPushResponse>>::send(
            &self.intasend,
            payload,
            service_path,
            request_method,
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
    pub currency: String,
    pub recipient: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MpesaStkPushResponse {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}

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
