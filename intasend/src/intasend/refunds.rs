#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;

use crate::{Intasend, Transaction};

use super::{FromJsonValue, Invoice, RequestClient, RequestMethods};

/// `Refunds` struct implements methods for facilitating:
/// listing all refunds made by an entity, creating new refunds for specific transactions with
/// its status **completed**, and also get a single specific refund made using the `chargeback_id`
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
/// // Chargebacks & Refunds API
/// let refunds: Refunds = intasend.refunds();
/// ```
///
#[derive(Deserialize, Debug)]
pub struct Refunds {
    pub(crate) intasend: Intasend,
}

impl Refunds {
    /// The `list` method returns all the refunds made by an entity from the IntaSend API
    ///
    /// ```rust
    /// // Chargebacks & Refunds API
    /// let refunds: Refunds = intasend.refunds();
    ///
    /// let refund_list = refunds.list().await?;
    /// println!("Checkout response: {:#?}", checkout_response);
    /// ```
    ///
    pub async fn list(&self) -> Result<RefundResponse, Error> {
        let service_path: &str = "/api/v1/chargebacks/";
        let request_method: RequestMethods = RequestMethods::GET;
        let payload: Option<RefundRequest> = None;

        let refunds = &self
            .intasend
            .send::<RefundRequest, RefundResponse>(payload, service_path, request_method)
            .await?;
        // println!("Json Response: {:#?}", refunds);

        Ok(refunds.clone())
    }

    /// The `create` method creates a new refund for a trnasaction whose status is **COMPLETE**.
    ///
    /// ```rust
    /// // Chargebacks & Refunds API
    /// let refunds: Refunds = intasend.refunds();
    ///
    /// let refund_request = RefundRequest {
    ///     amount: 100,
    ///     invoice_id: "RXX5P8R".to_string(),
    ///     currency: "USD".to_string(),
    ///     recipient: "recipient".to_string(),
    ///     method: "M-PESA".to_string(),
    /// };
    ///
    /// let created_refund: Refund = refunds.create(refund_request).await?;
    /// println!("[#] Created refund: {:?}", created_refund);
    /// ```
    ///
    pub async fn create(&self, payload: RefundRequest) -> Result<Refund, Error> {
        let service_path: &str = "/api/v1/chargebacks/";
        let request_method: RequestMethods = RequestMethods::POST;

        let created_refund = &self
            .intasend
            .send::<RefundRequest, Refund>(Some(payload), service_path, request_method)
            .await?;
        // println!("Json Response: {:#?}", created_refund);

        Ok(created_refund.clone())
    }

    /// The `get` method returns the specific refund with the specified `chargeback_id`.
    ///
    /// ```rust
    /// // Chargebacks & Refunds API
    /// let refunds: Refunds = intasend.refunds();
    ///
    /// let refund_request = RefundRequest {
    ///     amount: 100,
    ///     invoice_id: "RXX5P8R".to_string(),
    ///     currency: "USD".to_string(),
    ///     recipient: "recipient".to_string(),
    ///     method: "M-PESA".to_string(),
    /// };
    ///
    /// let created_refund: Refund = refunds.create(refund_request).await?;
    /// println!("[#] Created refund: {:?}", created_refund);
    ///
    /// let specific_refund = refunds.get("chargeback_id".to_string()).await?;
    /// println!("[#] Refund: {:?}", specific_refund);
    /// ```
    ///
    pub async fn get(&self, chargeback_id: String) -> Result<Refund, Error> {
        let service_path: &str = &format!("/api/v1/chargebacks/{}/", chargeback_id);
        let request_method: RequestMethods = RequestMethods::GET;

        let specific_refund = &self
            .intasend
            .send::<RefundRequest, Refund>(None, service_path, request_method)
            .await?;
        // println!("Json Response: {:#?}", specific_refund);

        Ok(specific_refund.clone())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Refund {
    // pub chargeback_id: String,
    // pub session_id: String,
    pub transaction: Transaction,
    pub amount: String,
    pub status: String,
    pub reason: RefundReason,
    pub resolution: Option<String>,
    pub staff_created: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum RefundReason {
    UnavailableService,
    DelayedDelivery,
    WrongService,
    DuplicatePayment,
    Other,
}

impl RefundReason {
    pub fn as_str(&self) -> String {
        match self {
            RefundReason::UnavailableService => "Unavailable service".to_string(),
            RefundReason::DelayedDelivery => "Delayed delivery".to_string(),
            RefundReason::WrongService => "Wrong service".to_string(),
            RefundReason::DuplicatePayment => "Duplicate payment".to_string(),
            RefundReason::Other => "Other".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RefundRequest {
    pub invoice_id: String,
    pub recipient: String,
    pub currency: String,
    pub method: String,
    pub amount: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct RefundResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Refund>,
}

// impl FromJsonValue for RefundResponse {
//     fn from_value(value: &JSON) -> Result<Self, anyhow::Error> {
//         let count = serde_json::from_value(value["count"].clone())?;
//         let next = serde_json::from_value(value["next"].clone())?;
//         let previous = serde_json::from_value(value["previous"].clone())?;
//         let results: Vec<Refund> =
//             serde_json::from_value(value.get("results").unwrap().clone()).unwrap();

//         Ok(RefundResponse {
//             count,
//             next,
//             previous,
//             results,
//         })
//     }
// }
