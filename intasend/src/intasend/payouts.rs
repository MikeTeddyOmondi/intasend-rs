#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::{Intasend, Wallet};

use super::{Currency, PayoutProvider, RequestClient, RequestMethods};

/// `PayoutsAPI` struct implements methods for facilitating:
/// Sending of funds to different recipients programatically.
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
/// // PayoutsAPI
/// let payout: Payouts = intasend.payout();
/// ```
///
#[derive(Deserialize, Debug)]
pub struct PayoutsAPI {
    pub(crate) intasend: Intasend,
}

impl PayoutsAPI {
    /// The `initiate` method controls the API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments
    pub async fn initiate(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let service_path: &str = "/api/v1/send-money/initiate/";
        let request_method = RequestMethods::Post;

        let payout = self
            .intasend
            .send::<PayoutRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    pub async fn mpesa_b2c(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::MpesaB2c); // PayoutProvider::MpesaB2c; 
        println!("mpesa_b2c payload: {:#?}", payload);
        let mpesa_payouts = self.initiate(payload).await?;
        Ok(mpesa_payouts)
    }

    pub async fn mpesa_b2b(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::MpesaB2b);
        let mpesa_b2b = self.initiate(payload).await?;
        Ok(mpesa_b2b)
    }

    pub async fn bank(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::Pesalink);
        let bank_payout = self.initiate(payload).await?;
        Ok(bank_payout)
    }

    pub async fn intasend(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::Intasend);
        let intasend_payout = self.initiate(payload).await?;
        Ok(intasend_payout)
    }

    pub async fn airtime(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::Airtime);
        let airtime = self.initiate(payload).await?;
        Ok(airtime)
    }

    pub async fn approve(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        // let client = Client::new();
        let service_path: &str = "/api/v1/send-money/approve/";
        let request_method = RequestMethods::Post;

        let payout = self
            .intasend
            .send::<PayoutRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    pub async fn status(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let service_path: &str = "/api/v1/send-money/status/";
        let request_method = RequestMethods::Get;

        let payout = self
            .intasend
            .send::<PayoutRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Payout {
    pub file_id: Option<String>,
    pub device_id: Option<String>,
    pub tracking_id: Option<String>,
    pub batch_reference: Option<String>,
    pub status: Option<String>,
    pub status_code: Option<String>,
    pub nonce: Option<String>,
    pub wallet: Option<Wallet>,
    pub transactions: Option<Vec<PayoutTransaction>>,
    pub charge_estimate: Option<String>,
    pub total_amount_estimate: Option<String>,
    pub total_amount: Option<String>,
    pub transactions_count: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutTransaction {
    pub status: Option<String>,
    pub status_code: Option<String>,
    pub request_reference_id: Option<String>,
    /// Beneficiary name as per Client Records
    pub name: Option<String>,
    /// Phone number, bank account number etc
    pub account: String,
    /// Optional ID number of beneficiary - MPesa transaction will be validated
    pub id_number: Option<String>,
    pub bank_code: Option<String>,
    pub amount: String,
    pub narrative: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayoutRequest {
    pub currency: Currency,
    pub provider: Option<PayoutProvider>, // PayoutProvider, 
    pub device_id: Option<String>,
    pub callback_url: Option<String>,
    pub batch_reference: Option<String>,
    pub requires_approval: PayoutApproval,
    pub transactions: Vec<PayoutTransaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PayoutApproval {
  Yes,
  No,
}