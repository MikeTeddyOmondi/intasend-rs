use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::Intasend;

#[derive(Deserialize, Debug)]
pub struct Payouts {
    pub(crate) intasend: Intasend,
}

impl Payouts {
    pub async fn initiate(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/send-money/initiate/", base_url))
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

        let payout: Payout = response?.json().await?;

        Ok(payout)
    }

    pub async fn mpesa(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.method = "MPESA-B2C".to_string();
        let mpesa_payouts = self.initiate(payload).await?;
        Ok(mpesa_payouts)
    }

    pub async fn mpesa_b2b(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.method = "MPESA-B2B".to_string();
        let mpesa_b2b = self.initiate(payload).await?;
        Ok(mpesa_b2b)
    }

    pub async fn bank(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.method = "PESALINK".to_string();
        let bank_payout = self.initiate(payload).await?;
        Ok(bank_payout)
    }

    pub async fn intasend(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.method = "INTASEND".to_string();
        let intasend_payout = self.initiate(payload).await?;
        Ok(intasend_payout)
    }

    pub async fn airtime(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let mut payload = payload;
        payload.method = "AIRTIME".to_string();
        let airtime = self.initiate(payload).await?;
        Ok(airtime)
    }

    pub async fn approve(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/send-money/approve/", base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.intasend.secret_key))
            .header("INTASEND_PUBLIC_API_KEY", self.intasend.publishable_key.clone())
            .json(&payload)
            .send()
            .await;

        let payout: Payout = response?.json().await?;

        Ok(payout)
    }

    pub async fn status(&self, payload: PayoutRequest) -> Result<Payout, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/send-money/status/", base_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.intasend.secret_key))
            .header("INTASEND_PUBLIC_API_KEY", self.intasend.publishable_key.clone())
            .json(&payload)
            .send()
            .await;

        let payout: Payout = response?.json().await?;

        Ok(payout)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Payout {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayoutRequest {
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}
