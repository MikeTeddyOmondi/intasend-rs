use anyhow::{Error as StdErr, Result};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::{Intasend, Wallet};

use super::{RequestClient, RequestMethods};

#[derive(Deserialize, Debug)]
pub struct Payouts {
    pub(crate) intasend: Intasend,
}

impl Payouts {
    pub async fn initiate(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let service_path: &str = "/api/v1/send-money/initiate/";
        let request_method = RequestMethods::POST;

        let payout = self
            .intasend
            .send::<PayoutRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    pub async fn mpesa_b2c(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let mut payload = payload;
        payload.provider = "MPESA-B2C".to_string();
        println!("mpesa_b2c payload: {:#?}", payload);
        let mpesa_payouts = self.initiate(payload).await?;
        Ok(mpesa_payouts)
    }

    pub async fn mpesa_b2b(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let mut payload = payload;
        payload.provider = "MPESA-B2B".to_string();
        let mpesa_b2b = self.initiate(payload).await?;
        Ok(mpesa_b2b)
    }

    pub async fn bank(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let mut payload = payload;
        payload.provider = "PESALINK".to_string();
        let bank_payout = self.initiate(payload).await?;
        Ok(bank_payout)
    }

    pub async fn intasend(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let mut payload = payload;
        payload.provider = "INTASEND".to_string();
        let intasend_payout = self.initiate(payload).await?;
        Ok(intasend_payout)
    }

    pub async fn airtime(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let mut payload = payload;
        payload.provider = "AIRTIME".to_string();
        let airtime = self.initiate(payload).await?;
        Ok(airtime)
    }

    pub async fn approve(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        // let client = Client::new();
        let service_path: &str = "/api/v1/send-money/approve/";
        let request_method = RequestMethods::POST;

        let payout = self
            .intasend
            .send::<PayoutRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    pub async fn status(&self, payload: PayoutRequest) -> Result<Payout, StdErr> {
        let service_path: &str = "/api/v1/send-money/status/";
        let request_method = RequestMethods::GET;

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
    pub currency: String,
    pub provider: String,
    pub device_id: Option<String>,
    pub callback_url: Option<String>,
    pub batch_reference: Option<String>,
    pub transactions: Vec<PayoutTransaction>,
}
