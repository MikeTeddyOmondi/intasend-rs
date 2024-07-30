use anyhow::Result;
use futures::stream::Next;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JSON};

use super::{Customer, RequestClient, RequestMethods};
use crate::{Currency, Intasend, Transaction};

/// `WalletsAPI` struct implements methods for facilitating:
/// listing all wallets managed with an API key, get details
/// of a specific wallet using `wallet_id`, create new `WORKING` 
/// wallet types as sub-accounts for an associated IntaSend API key
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
/// // WalletsAPI
/// let wallets_api: WalletsAPI = intasend.wallets();
/// ```
///
#[derive(Debug)]
pub struct WalletsAPI {
    pub(crate) intasend: Intasend,
}

impl WalletsAPI {
    /// The `list` (Wallets API) enables you to access Wallets owned by you or
    /// created in your account.
    ///
    /// ```rust
    /// // WalletsAPI
    /// let wallets_api: WalletsAPI = intasend.wallets();
    ///
    /// let wallet_list_info: WalletListResponse = wallets_api.list().await?;
    /// println!("[#] Wallet List Info: {:#?}", wallet_list_info);
    /// ```
    ///
    pub async fn list(&self) -> Result<WalletListResponse> {
        let service_path: &str = "/api/v1/wallets/";
        let request_method: RequestMethods = RequestMethods::Get;

        let wallet_list_info = &self
            .intasend
            .send::<WalletListRequest, WalletListResponse>(None, service_path, request_method)
            .await?;

        // let wallet_list_info: Vec<Wallet> = response?.json().await?;

        Ok(wallet_list_info.clone())
    }
    
    /// The `details` (Wallets API) enables you to access Wallets owned by you or
    /// created in your account.
    ///
    /// ```rust
    /// // WalletsAPI
    /// let wallets_api: WalletsAPI = intasend.wallets();
    ///
    /// let wallet_details: Wallet = wallets_api.details(wallet_id).await?;
    /// println!("[#] Wallet Details Info: {:#?}", wallet_list_info);
    /// ```
    ///
    pub async fn details(&self, wallet_id: String) -> Result<Wallet> {
        let service_path: &str = &format!("/api/v1/wallets/{}", wallet_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let wallet_details_info = &self
            .intasend
            .send::<WalletDetailsRequest, Wallet>(None, service_path, request_method)
            .await?;

        // let wallet_list_info: Vec<Wallet> = response?.json().await?;

        Ok(wallet_details_info.clone())
    }

    /// The `create` (WalletsAPI) will help you to create and manage **WORKING** wallets. 
    /// 
    /// By default, all IntaSend accounts have **SETTLEMENT** accounts which act as main accounts. 
    /// 
    /// **WORKING** wallets are basically sub-accounts that you can use to isolate your 
    /// customers'/merchants' funds. 
    /// 
    /// Each customer can have their own wallets within IntaSend that you will manage on their behalf.
    /// 
    /// ```rust
    /// // WalletsAPI
    /// let wallets_api: WalletsAPI = intasend.wallets();
    /// 
    /// let payload = WalletCreateDetails {
    ///     currency: Currency::KES,
    ///     wallet_type: WalletType::Working,
    ///     can_disburse: true,
    ///     label: "wallet-label".to_string(),
    /// };
    ///
    /// let created_wallet: Wallet = wallets_api.create(payload).await?;
    /// println!("[#] Wallet Details Info: {:#?}", wallet_list_info);
    /// ```
    pub async fn create(&self, payload: WalletCreateDetails) -> Result<Wallet> {
      let service_path: &str = "/api/v1/wallets/";
      let request_method: RequestMethods = RequestMethods::Post;

      let created_wallet = &self
          .intasend
          .send::<WalletCreateDetails, Wallet>(Some(payload), service_path, request_method)
          .await?;

      Ok(created_wallet.clone())
    }

    pub async fn intra_transfer(
        &self,
        source_id: String,
        destination_id: String,
        amount: u32,
        narrative: String,
    ) -> Result<Wallet> {
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

    pub async fn get(&self, wallet_id: String) -> Result<Wallet> {
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

    pub async fn transactions(&self, wallet_id: String) -> Result<Vec<Transaction>> {
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

    pub async fn fund_mpesa(&self, payload: FundCheckoutRequest) -> Result<Wallet> {
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

    pub async fn fund_checkout(&self, payload: FundCheckoutRequest) -> Result<Wallet> {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub wallet_id: String,
    pub label: String,
    pub can_disburse: bool,
    pub currency: Currency,
    pub wallet_type: WalletType,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WalletListRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletListResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Wallet>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletDetailsRequest {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletCreateDetails {
  pub currency: Currency,
  #[serde(default = "default_wallet_type")]
  pub wallet_type: WalletType,
  pub can_disburse: bool,
  pub label: String,
}

fn default_wallet_type() -> WalletType {
    WalletType::Working
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundCheckoutRequest {
    pub amount: u32,
    pub currency: String,
    pub recipient: String,
    pub method: String,
}
