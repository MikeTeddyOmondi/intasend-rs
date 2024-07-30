#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::Intasend;

use super::{Customer, Invoice, RequestClient, RequestMethods, Tarrif};

/// `Collection` struct implements methods for facilitating:
/// Mpesa Express for merchant initiated online payments
/// 1. M-Pesa STK Push
/// 2. Querying status of transactions
/// 
/// ```rust
/// // Load .env file
/// dotenv().ok();
///
/// let intasend_public_key = env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
/// let intasend_secret_key = env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
///
/// // Intasend Client
/// let intasend = Intasend::new(
///    intasend_public_key,
///    intasend_secret_key,
///     true,
/// );
///
/// // Collection API
/// let collection: Collection = intasend.collection();
/// ```
///
#[derive(Debug)]
pub struct CollectionsAPI {
    pub(crate) intasend: Intasend,
}

impl CollectionsAPI {
    /// The `mpesa_stk_push` method initiates an M-pesa transaction to trigger an authorization
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
        let request_method: RequestMethods = RequestMethods::Post;

        // let json_response = <Intasend as RequestClient<MpesaStkPushRequest>>::send(
        //     &self.intasend,
        //     Some(payload),
        //     service_path,
        //     request_method,
        // )
        // .await?;
        let mpesa_stk_push_response = &self
            .intasend
            .send::<MpesaStkPushRequest, MpesaStkPushResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;
        // println!("Json Response: {:#?}", json_response);

        // let mpesa_stk_push_response = MpesaStkPushResponse::from_value(&json_response).unwrap();
        // println!("Response: {:#?}", mpesa_stk_push_response);

        // let mut invoice_object: Invoice = serde_json::from_value(json_response["invoice"].clone())?;
        // println!("Invoice: {:#?}", invoice_object);

        Ok(mpesa_stk_push_response.clone())
    }

    /// The `status` method initiates an M-pesa query about an initiated transaction
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
    /// let stkpushstatus_req: StkPushStatusRequest = StkPushStatusRequest {
    ///     invoice_id: stkpush_response.invoice.unwrap().invoice_id,
    ///     checkout_id: None,
    ///     signature: None,
    /// };
    ///
    /// let stkpushstatus: StkPushStatusResponse = collection.status(stkpushstatus_req).await?;
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
        let request_method: RequestMethods = RequestMethods::Post;

        // let json_response = <Intasend as RequestClient<StkPushStatusRequest>>::send(
        //     &self.intasend,
        //     Some(payload),
        //     service_path,
        //     request_method,
        // )
        // .await?;
        let status_response = &self
            .intasend
            .send::<StkPushStatusRequest, StkPushStatusResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;
        // println!("Json Response: {:#?}", json_response);

        // let status_response = StkPushStatusResponse::from_value(&json_response).unwrap();
        // println!("Status Response: {:#?}", status_response);

        Ok(status_response.clone())
    }
}

/// `MPesaSTKPushRequest` Struct - `CollectionsAPI`
#[derive(Deserialize, Serialize, Debug)]
pub struct MpesaStkPushRequest {
    pub amount: Decimal,
    pub phone_number: String,
    pub api_ref: Option<String>,
    pub wallet_id: Option<String>,
}

/// `MpesaStkPushResponse` Struct - `CollectionsAPI`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MpesaStkPushResponse {
    pub invoice: Option<Invoice>,
    pub customer: Option<Customer>,
    pub payment_link: Option<String>,
    pub refundable: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// `StkPushStatusRequest` Struct - `CollectionsAPI`
#[derive(Deserialize, Serialize, Debug)]
pub struct StkPushStatusRequest {
    pub invoice_id: String,
    pub checkout_id: Option<String>,
    pub signature: Option<String>,
}

/// `StkPushStatusResponse` Struct - `CollectionsAPI`
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct StkPushStatusResponse {
    pub invoice: Option<Invoice>,
    pub meta: Meta,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Meta {
    pub id: String,
    pub customer_comment: Option<String>,
    pub payment_link: Option<PaymentLink>,
    pub customer: Option<Customer>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
