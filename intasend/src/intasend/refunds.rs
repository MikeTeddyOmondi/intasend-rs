use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::Intasend;

#[derive(Deserialize, Debug)]
pub struct Refunds {
    pub(crate) intasend: Intasend,
}

impl Refunds {
    pub async fn list(&self) -> Result<Vec<Refunds>, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .get(&format!("{}/api/v1/chargebacks/", base_url))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.intasend.secret_key),
            )
            .header(
                "INTASEND_PUBLIC_API_KEY",
                self.intasend.publishable_key.clone(),
            )
            .send()
            .await;

        let refunds: Vec<Refunds> = response?.json().await?;

        Ok(refunds)
    }

    pub async fn create(&self, payload: RefundRequest) -> Result<Refunds, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/chargebacks/", base_url))
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

        let refund: Refunds = response?.json().await?;

        Ok(refund)
    }

    pub async fn get(&self, chargeback_id: String) -> Result<Refunds, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .get(&format!(
                "{}/api/v1/chargebacks/{}/",
                base_url, chargeback_id
            ))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.intasend.secret_key),
            )
            .header(
                "INTASEND_PUBLIC_API_KEY",
                self.intasend.publishable_key.clone(),
            )
            .send()
            .await;

        let refund: Refunds = response?.json().await?;

        Ok(refund)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Refund {
    pub id: String,
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}

#[derive(Deserialize, Serialize)]
pub struct RefundRequest {
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}
