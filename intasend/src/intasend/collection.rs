#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{FromJsonValue, RequestClient, RequestMethods, Tarrif};

/// Collection struct implements methods for facilitating:
/// Mpesa Express for merchant initiated online payments
/// 1. M-Pesa STK Push
/// 2. Querying status of transactions
/// ```rust
/// // Intasend client
/// let intasend = Intasend::new(
///     publishable_key: "ISPubKey_test_c1f90113-3dbb-4201-9b88-f1c2d3056e5c".to_string(),
///     secret_key: "ISSecretKey_test_5527b085-40b6-460a-9c31-25d58a204d20".to_string(),
///     test_mode: true
/// );
///
/// // Collection API
/// let collection: Collection = intasend.collection();
/// ```
///
#[derive(Debug)]
pub struct Collection {
    pub(crate) intasend: Intasend,
}

impl Collection {
    /// The mpesa_stk_push method initiates an M-pesa transaction to trigger an authorization
    /// from the end user to accept the transaction.
    ///
    /// ```rust
    /// // Collection API
    /// let collection: Collection = intasend.collection();
    ///
    /// let stkpush_request = MpesaStkPushRequest {
    ///     amount: 10,
    ///     phone_number: "254712345678".to_string(),
    ///     api_ref: None,
    ///     wallet_id: None,
    /// };
    ///
    /// let stkpush_response: MpesaStkPushResponse = collection.mpesa_stk_push(stkpush_request).await?;
    /// println!("[#] Mpesa STK push: {:#?}", stkpush_response);
    /// ```
    ///
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
        // println!("Json Response: {:#?}", json_response);

        let mpesa_stk_push_response = MpesaStkPushResponse::from_value(&json_response).unwrap();
        // println!("Response: {:#?}", mpesa_stk_push_response);

        // let mut invoice_object: Invoice = serde_json::from_value(json_response["invoice"].clone())?;
        // println!("Invoice: {:#?}", invoice_object);

        Ok(mpesa_stk_push_response)
    }

    /// The status method initiates an M-pesa query about an initiated transaction
    /// authorised from the end user.
    ///
    /// ```rust
    /// // Collection API
    /// let collection: Collection = intasend.collection();
    ///
    /// let stkpush_request = MpesaStkPushRequest {
    ///     amount: 10,
    ///     phone_number: "254712345678".to_string(),
    ///     api_ref: None,
    ///     wallet_id: None,
    /// };
    ///
    /// let stkpush_response: MpesaStkPushResponse = collection.mpesa_stk_push(stkpush_request).await?;
    /// println!("[#] Mpesa STK push: {:#?}", stkpush_response);
    ///
    /// println!("[#] Waiting for the collection response...");
    /// tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    ///
    /// let stkpushstatus_req: StatusRequest = StatusRequest {
    ///     invoice_id: stkpush_response.invoice.unwrap().invoice_id,
    ///     checkout_id: None,
    ///     signature: None,
    /// };
    ///
    /// let stkpushstatus = collection.status(stkpushstatus_req).await?;
    /// println!(
    ///     "[#] Mpesa STK Push Status Response: {:#?}",
    ///     stkpushstatus
    /// );
    ///
    /// ```
    ///
    pub async fn status(
        &self,
        payload: StkPushStatusRequest,
    ) -> Result<StkPushStatusResponse, Error> {
        let service_path: &str = "/api/v1/payment/status/";
        let request_method: RequestMethods = RequestMethods::POST;

        let json_response = <Intasend as RequestClient<StkPushStatusRequest>>::send(
            &self.intasend,
            payload,
            service_path,
            request_method,
        )
        .await?;
        // println!("Json Response: {:#?}", json_response);

        let status_response = StkPushStatusResponse::from_value(&json_response).unwrap();
        // println!("Status Response: {:#?}", status_response);

        Ok(status_response)
    }
}

/// MPesaSTKPushRequest Struct - Collection API
#[derive(Deserialize, Serialize, Debug)]
pub struct MpesaStkPushRequest {
    pub amount: u32,
    pub phone_number: String,
    pub api_ref: Option<String>,
    pub wallet_id: Option<String>,
}

/// MpesaStkPushResponse Struct - Collection API
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
    pub invoice_id: String,
    pub state: String,
    pub provider: String,
    pub charges: String,
    // #[serde(with = "rust_decimal::serde::str")]
    pub net_amount: Decimal,
    pub currency: String,
    // #[serde(with = "rust_decimal::serde::str")]
    pub value: Decimal,
    pub account: String,
    pub api_ref: Option<String>,
    pub mpesa_reference: Option<String>,
    pub host: String,
    pub card_info: CardInfo,
    pub retry_count: u32,
    pub failed_reason: Option<String>,
    pub failed_code: Option<String>,
    pub failed_code_link: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: String,
    pub phone_number: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub zipcode: Option<String>,
    pub provider: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo {
    pub bin_country: Option<String>,
    pub card_type: Option<String>,
}

/// StatusRequest Struct - Collection API
#[derive(Deserialize, Serialize, Debug)]
pub struct StkPushStatusRequest {
    pub invoice_id: String,
    pub checkout_id: Option<String>,
    pub signature: Option<String>,
}

/// StatusResponse Struct - Collection API
#[derive(Deserialize, Serialize, Debug)]
pub struct StkPushStatusResponse {
    pub invoice: Option<Invoice>,
    pub meta: Meta,
}

impl FromJsonValue for StkPushStatusResponse {
    fn from_value(value: &JSON) -> Result<Self, anyhow::Error> {
        let invoice: Option<Invoice> =
            serde_json::from_value(value.get("invoice").unwrap().clone()).unwrap();
        // serde_json::from_value(value.get("meta").unwrap().clone()).unwrap();
        let meta: Meta = serde_json::from_value(value["meta"].clone())?;

        Ok::<StkPushStatusResponse, Error>(StkPushStatusResponse {
            invoice,
            meta,
        })
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Meta {
    pub id: String,
    pub customer_comment: Option<String>,
    pub payment_link: Option<PaymentLink>,
    pub customer: Option<Customer>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentLink {
    pub id: String,
    pub title: String,
    pub is_active: bool,
    pub redirect_url: Option<String>,
    pub amount: Decimal,
    pub usage_limit: Decimal,
    pub qrcode_file: Option<String>,
    pub url: String,
    pub currency: String,
    pub mobile_tarrif: Tarrif,
    pub card_tarrif: Tarrif,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
