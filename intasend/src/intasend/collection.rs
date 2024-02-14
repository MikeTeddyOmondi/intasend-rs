#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{FromJsonValue, RequestClient, RequestMethods};

#[derive(Debug)]
pub struct Collection {
    pub(crate) intasend: Intasend,
}

impl Collection {
    pub async fn mpesa_stk_push(
        &self,
        payload: MpesaStkPushRequest,
    ) -> Result<MpesaStkPushResponse, Error> {
        let service_path: &str = "/api/v1/payment/mpesa-stk-push/";
        let request_method: RequestMethods = RequestMethods::POST;

        let json_response = <Intasend as RequestClient<MpesaStkPushRequest>>::send(
            &self.intasend,
            payload,
            service_path,
            request_method,
        )
        .await?;
        println!("Json Response: {:#?}", json_response);

        // Deserialize the JSON response into a Value
        // let json_value: Value = serde_json::from_str(&json_response).unwrap();

        // // Extract fields and create MpesaStkPushResponse struct
        // let mpesa_stk_push_response = MpesaStkPushResponse {
        //     invoice: serde_json::from_value(json_response["invoice"].clone()).ok(),
        //     customer: serde_json::from_value(json_response["customer"].clone()).ok(),
        //     payment_link: json_response["payment_link"]
        //         .as_str()
        //         .map(|s| s.to_string()),
        //     refundable: json_response["refundable"].as_bool().unwrap_or(false),
        //     created_at: json_response["created_at"]
        //         .as_str()
        //         .unwrap_or_default()
        //         .to_string(),
        //     updated_at: json_response["updated_at"]
        //         .as_str()
        //         .unwrap_or_default()
        //         .to_string(),
        // };

        // println!("{:#?}", mpesa_stk_push_response);

        // match MpesaStkPushResponse::from_value(json_response) {
        //     Ok(response) => {
        //         println!("MpesaStkPushResponse created: {:?}", response);
        //         Ok(response)
        //     }
        //     Err(err) => {
        //         println!("Error creating MpesaStkPushResponse: {:?}", err);
        //         Err(std::fmt::Error.into())
        //     }
        // }

        let resp = MpesaStkPushResponse::from_value(&json_response).unwrap();
        println!("Response: {:#?}", resp);
        let mut invoice_object: Invoice = serde_json::from_value(json_response["invoice"].clone())?;
        // invoice_object.net_amount = json_response["net_amount"]
        //     .as_str()
        //     .unwrap()
        //     .parse::<Decimal>()?;
        // invoice_object.value = json_response["value"]
        //     .as_str()
        //     .unwrap()
        //     .parse::<Decimal>()?;
        println!("Invoice: {:#?}", invoice_object);
        Ok(resp)

        //  json_response.get("invoice").map_or_else(|| None,|invoice_value| Some(invoice_value.into()))

        // println!("{:#?}", serde_json::from_value(json_response["invoice"].clone())?);
        // let invoice = json_response["invoice"].as_object().clone().unwrap();
        // println!("{:#?}", invoice);

        // // serde_json::from_value(Value::Object(invoice.clone()))?;
        // let mut invoice_object: Invoice = serde_json::from_value(json_response["invoice"].clone())?;
        // invoice_object.value = json_response["value"].as_str().unwrap().parse::<f64>()?;
        // println!("{:#?}", invoice_object);

        // let mpesa_stk_push_response = MpesaStkPushResponse {
        //     invoice: None,
        //     customer: None,
        //     payment_link: None,
        //     refundable: true,
        //     created_at: "2024-01-01T00:00:00".to_string(),
        //     updated_at: "2024-01-01T00:00:00".to_string(),
        // };

        // let mpesa_stk_push_response = json_response;
        // Ok(mpesa_stk_push_response)
    }

    pub async fn status(&self, payload: StatusRequest) -> Result<StatusResponse, Error> {
        let service_path: &str = "/api/v1/payment/mpesa-stk-push/";
        let request_method: RequestMethods = RequestMethods::POST;

        let json_response = <Intasend as RequestClient<StatusRequest>>::send(
            &self.intasend,
            payload,
            service_path,
            request_method,
        )
        .await?;
        // println!("Json Response: {:?}", json_response);
        println!("Json Response: {:#?}", json_response);

        let status_response = StatusResponse::from_value(&json_response).unwrap();
        println!("Response: {:#?}", status_response);

        Ok(status_response)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MpesaStkPushRequest {
    pub amount: u32,
    pub phone_number: String,
    pub api_ref: Option<String>,
    pub wallet_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MpesaStkPushResponse {
    pub invoice: Option<Invoice>,
    pub customer: Option<Customer>,
    pub payment_link: Option<String>,
    pub refundable: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl FromJsonValue for MpesaStkPushResponse {
    fn from_value(value: &JSON) -> Result<Self, anyhow::Error> {
        // let invoice = value["invoice"].clone().into();
        let invoice: Option<Invoice> =
            serde_json::from_value(value.get("invoice").unwrap().clone()).unwrap();
        // let customer = value["customer"].clone().into();
        let customer: Option<Customer> =
            serde_json::from_value(value.get("customer").unwrap().clone()).unwrap();
        let payment_link = value
            .get("payment_link")
            .unwrap()
            .as_str()
            .map(|s| s.to_string());
        let refundable = value
            .get("refundable")
            .unwrap()
            .as_bool()
            .ok_or(Error::msg("Refundable not found"))?;
        let created_at = value
            .get("created_at")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("create_at field at not found"))?
            .to_string();
        let updated_at = value
            .get("updated_at")
            .unwrap()
            .as_str()
            .ok_or(Error::msg("updated_at field not found"))?
            .to_string();

        Ok::<MpesaStkPushResponse, Error>(MpesaStkPushResponse {
            invoice,
            customer,
            payment_link,
            refundable,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    invoice_id: String,
    state: String,
    provider: String,
    charges: String,
    // #[serde(with = "rust_decimal::serde::str")]
    net_amount: Decimal,
    currency: String,
    // #[serde(with = "rust_decimal::serde::str")]
    value: Decimal,
    account: String,
    api_ref: Option<String>,
    mpesa_reference: Option<String>,
    host: String,
    card_info: CardInfo,
    retry_count: u32,
    failed_reason: Option<String>,
    failed_code: Option<u32>,
    failed_code_link: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo {
    bin_country: Option<String>,
    card_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusRequest {
    pub invoice_id: String,
    pub signature: Option<String>,
    pub checkout_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StatusResponse {
    pub invoice: Option<Invoice>,
    // pub amount: u32,
    // pub currency: String,
    // pub recipient: String,
    // pub method: String,
}

impl FromJsonValue for StatusResponse {
    fn from_value(value: &JSON) -> Result<Self, anyhow::Error> {
        let invoice: Option<Invoice> =
            serde_json::from_value(value.get("invoice").unwrap().clone()).unwrap();
        // let customer: Option<Customer> =
        //     serde_json::from_value(value.get("customer").unwrap().clone()).unwrap();
        // let payment_link = value
        //     .get("payment_link")
        //     .unwrap()
        //     .as_str()
        //     .map(|s| s.to_string());
        // let refundable = value
        //     .get("refundable")
        //     .unwrap()
        //     .as_bool()
        //     .ok_or(Error::msg("Refundable not found"))?;
        // let created_at = value
        //     .get("created_at")
        //     .unwrap()
        //     .as_str()
        //     .ok_or(Error::msg("create_at field at not found"))?
        //     .to_string();
        // let updated_at = value
        //     .get("updated_at")
        //     .unwrap()
        //     .as_str()
        //     .ok_or(Error::msg("updated_at field not found"))?
        //     .to_string();

        Ok::<StatusResponse, Error>(StatusResponse {
            invoice,
            // customer,
            // payment_link,
            // refundable,
            // created_at,
            // updated_at,
            // id: todo!(),
            // amount: todo!(),
            // currency: todo!(),
            // recipient: todo!(),
            // method: todo!(),
        })
    }
}