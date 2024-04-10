use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Currency, Intasend, Transaction};

pub struct Wallets {
    pub(crate) intasend: Intasend,
}

impl Wallets {
    pub async fn list(&self) -> Result<Vec<Wallet>, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .get(&format!("{}/api/v1/wallets/", base_url))
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

        let wallets: Vec<Wallet> = response?.json().await?;

        Ok(wallets)
    }

    pub async fn create(&self, payload: Wallet) -> Result<Wallet, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/wallets/", base_url))
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

        let wallet: Wallet = response?.json().await?;

        Ok(wallet)
    }

    pub async fn intra_transfer(
        &self,
        source_id: String,
        destination_id: String,
        amount: u32,
        narrative: String,
    ) -> Result<Wallet, Error> {
        let payload = json!({
            "wallet_id": destination_id,
            "amount": amount,
            "narrative": narrative,
        });

        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!(
                "{}/api/v1/wallets/{}/intra_transfer/",
                base_url, source_id
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
            .json(&payload)
            .send()
            .await;

        let wallet: Wallet = response?.json().await?;

        Ok(wallet)
    }

    pub async fn get(&self, wallet_id: String) -> Result<Wallet, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .get(&format!("{}/api/v1/wallets/{}/", base_url, wallet_id))
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

        let wallet: Wallet = response?.json().await?;

        Ok(wallet)
    }

    pub async fn transactions(&self, wallet_id: String) -> Result<Vec<Transaction>, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .get(&format!(
                "{}/api/v1/wallets/{}/transactions/",
                base_url, wallet_id
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

        let transactions: Vec<Transaction> = response?.json().await?;

        Ok(transactions)
    }

    pub async fn fund_mpesa(&self, payload: FundCheckoutRequest) -> Result<Wallet, Error> {
        let mut payload = payload;
        payload.method = "M-PESA".to_string();
        payload.currency = "KES".to_string();

        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/payment/mpesa-stk-push/", base_url))
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

        let wallet: Wallet = response?.json().await?;

        Ok(wallet)
    }

    pub async fn fund_checkout(&self, payload: FundCheckoutRequest) -> Result<Wallet, Error> {
        let client = Client::new();

        let base_url = if self.intasend.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        let response = client
            .post(&format!("{}/api/v1/checkout/", base_url))
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

        let wallet: Wallet = response?.json().await?;

        Ok(wallet)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub wallet_id: String,
    pub label: String,
    pub can_disburse: bool,
    pub currency: Currency,
    pub wallet_type: WalletType,
    pub current_balance: String,
    pub available_balance: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WalletType {
   Settlement,
   Working, 
}

impl WalletType {
    pub fn as_str(&self) -> String {
        match self {
            WalletType::Settlement => "SETTLEMENT".to_string(),
            WalletType::Working => "WORKING".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FundCheckoutRequest {
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}
