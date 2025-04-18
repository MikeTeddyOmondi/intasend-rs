use anyhow::Result;
use futures::stream::Next;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JSON};

use crate::{Currency, Intasend, Transaction};

use super::{Customer, Invoice, Provider, RequestClient, RequestMethods};

/// `WalletsAPI` struct implements methods for facilitating:
/// listing all wallets managed with an API key, get details
/// of a specific wallet using `wallet_id`, create new `WORKING`
/// wallet types as sub-accounts for an associated IntaSend API key
///
/// ```rust
/// // Load .env file
/// dotenvy::dotenv().ok();
///
/// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
/// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
///
/// // Intasend Client
/// let intasend = intasend::Intasend::new(
///    intasend_public_key,
///    intasend_secret_key,
///     true,
/// );
///
/// // WalletsAPI
/// let wallets_api: intasend::WalletsAPI = intasend.wallets();
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
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    ///
    /// let wallet_list_info: intasend::WalletListResponse = wallets_api.list().await?;
    /// println!("[#] Wallet List Info: {:#?}", wallet_list_info);
    /// 
    /// Ok(())
    /// }
    /// ```
    ///
    pub async fn list(&self) -> Result<WalletListResponse> {
        let service_path: &str = "/api/v1/wallets/";
        let request_method: RequestMethods = RequestMethods::Get;

        let wallet_list_info = &self
            .intasend
            .send::<WalletListRequest, WalletListResponse>(None, service_path, request_method)
            .await?;

        Ok(wallet_list_info.clone())
    }

    /// The `details` (Wallets API) enables you to access wallet's details.
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    /// let wallet_id = String::from("wallet_id");
    /// let wallet_details: intasend::Wallet = wallets_api.details(wallet_id).await?;
    /// println!("[#] Wallet Details Info: {:#?}", wallet_details);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn details(&self, wallet_id: String) -> Result<Wallet> {
        let service_path: &str = &format!("/api/v1/wallets/{}", wallet_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let wallet_details_info = &self
            .intasend
            .send::<WalletDetailsRequest, Wallet>(None, service_path, request_method)
            .await?;

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
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    ///
    /// let payload = intasend::WalletCreateDetails {
    ///     currency: intasend::Currency::Kes,
    ///     wallet_type: intasend::WalletType::Working,
    ///     can_disburse: true,
    ///     label: "wallet-label".to_string(),
    /// };
    ///
    /// let created_wallet: intasend::Wallet = wallets_api.create(payload).await?;
    /// println!("[#] Wallet Details Info: {:#?}", created_wallet);
    /// 
    /// Ok(())
    /// # }
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

    /// The `transactions` (WalletsAPI) enables you to get all the transactions from a specific Wallet.
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    /// let wallet_id = String::from("wallet_id");
    /// let wallet_transanctions = wallets_api.transactions(wallet_id).await?;
    /// println!("[#] Wallet Transactions: {:#?}", wallet_transanctions);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn transactions(&self, wallet_id: String) -> Result<WalletTransactionsResponse> {
        let service_path: &str = &format!("/api/v1/wallets/{}/transactions/", wallet_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let transactions = &self
            .intasend
            .send::<WalletTransanctionsRequest, WalletTransactionsResponse>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(transactions.clone())
    }

    /// The `intra_transfer` (WalletsAPI) enables you to send funds within a specific IntaSend Wallet
    /// to another IntaSend wallet i.e internal wallet to wallet transfers
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    ///
    /// let intra_transfer_payload = intasend::WalletIntraTransferRequest {
    ///     wallet_id: "Y7ERXJQ".to_string(),
    ///     amount: rust_decimal::Decimal::new(1000, 2),
    ///     narrative: "fund raising".to_string(),
    /// };
    /// 
    /// let source_wallet_id = String::from("Y7ERXJQ");
    /// 
    /// let wallets_transfer_response = wallets_api.intra_transfer(source_wallet_id, intra_transfer_payload).await?;
    /// println!("[#] Wallet Intra Transfer Response: {:#?}", wallets_transfer_response);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn intra_transfer(
        &self,
        source_wallet_id: String,
        payload: WalletIntraTransferRequest,
    ) -> Result<WalletIntraTransferResponse> {
        let service_path: &str = &format!("/api/v1/wallets/{}/intra_transfer/", source_wallet_id);
        let request_method: RequestMethods = RequestMethods::Post;

        let intra_transfer_response = &self
            .intasend
            .send::<WalletIntraTransferRequest, WalletIntraTransferResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(intra_transfer_response.clone())
    }

    /// The `fund_mpesa` (WalletsAPI) enables you to fund a specific IntaSend Wallet
    ///  using M-pesa.
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    /// 
    /// let fund_mpesa_payload = intasend::FundMpesaRequest {
    ///   method: intasend::Provider::Mpesa,
    ///   currency: intasend::Currency::Kes,
    ///   amount: rust_decimal::Decimal::new(1000, 2),
    ///   wallet_id: "Y7ELXJQ".to_string(),
    ///   phone_number: "254717135176".to_string(),
    /// };
    /// let fund_mpesa_response = wallets_api.fund_mpesa(fund_mpesa_payload).await?;
    /// println!("[#] Wallet Fund Mpesa Response: {:#?}", fund_mpesa_response);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn fund_mpesa(&self, payload: FundMpesaRequest) -> Result<FundMpesaResponse> {
        let service_path: &str = "/api/v1/payment/mpesa-stk-push/";
        let request_method: RequestMethods = RequestMethods::Post;

        let mut payload = payload;
        payload.method = Provider::Mpesa;
        payload.currency = Currency::Kes;

        let fund_mpesa_response = &self
            .intasend
            .send::<FundMpesaRequest, FundMpesaResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(fund_mpesa_response.clone())
    }

    /// The `fund_checkout` (WalletsAPI) enables you to fund a specific IntaSend Wallet
    ///  using checkout links.
    ///
    /// ```rust
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// dotenvy::dotenv().ok();
    ///
    /// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    /// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
    ///
    /// // Intasend Client
    /// let intasend = intasend::Intasend::new(
    ///    intasend_public_key,
    ///    intasend_secret_key,
    ///     true,
    /// );
    /// 
    /// // WalletsAPI
    /// let wallets_api: intasend::WalletsAPI = intasend.wallets();
    /// 
    /// let fund_checkout_req = intasend::FundCheckoutRequest {
    ///     first_name: Some("Foo".to_string()),
    ///     last_name: Some("Bar".to_string()),
    ///     email: Some("foobar@baz.com".to_string()),
    ///     method: Some(intasend::Provider::Bank),
    ///     amount: rust_decimal::Decimal::new(100000, 2), // 1000.00
    ///     currency: intasend::Currency::Kes,
    ///     wallet_id: "Y7ELXJQ".to_string(),
    ///     api_ref: None,
    ///     redirect_url: None,
    /// };
    ///
    /// let fund_checkout_response = wallets_api.fund_checkout(fund_checkout_req).await?;
    /// println!("[#] Fund Checkout response: {:#?}", fund_checkout_response);
    /// 
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn fund_checkout(
        &self,
        payload: FundCheckoutRequest,
    ) -> Result<FundCheckoutResponse> {
        let service_path: &str = "/api/v1/checkout/";
        let request_method: RequestMethods = RequestMethods::Post;

        let fund_checkout_response = &self
            .intasend
            .send_client_request::<FundCheckoutRequest, FundCheckoutResponse>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(fund_checkout_response.clone())
    }
}

/// `Wallet` struct 
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

/// `WalletType` enum
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum WalletType {
    Settlement,
    Working,
}

/// `WalletListRequest` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletListRequest {}

/// `WalletListResponse` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletListResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Wallet>,
}

/// `WalletDetailsRequest`struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletDetailsRequest {}

/// `WalletCreateDetails` struct
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

/// `WalletTransanctionsRequest` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
struct WalletTransanctionsRequest {}

/// `WalletTransactionsResponse` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletTransactionsResponse {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Transaction>,
}

/// `WalletIntraTransferRequest` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletIntraTransferRequest {
    pub wallet_id: String,
    pub amount: Decimal,
    pub narrative: String,
}

/// `WalletIntraTransferResponse` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletIntraTransferResponse {
    pub origin: Wallet,
    pub destination: Wallet,
}

/// `FundMpesaRequest` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundMpesaRequest {
    pub amount: Decimal,
    pub method: Provider,
    pub wallet_id: String,
    pub currency: Currency,
    pub phone_number: String,
}

/// `FundMpesaResponse` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundMpesaResponse {
    pub invoice: Option<Invoice>,
    pub customer: Option<Customer>,
    pub payment_link: Option<String>,
    pub refundable: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// `FundCheckoutRequest` 
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundCheckoutRequest {
    pub amount: Decimal,
    pub wallet_id: String,
    pub currency: Currency,
    pub email: Option<String>,
    pub api_ref: Option<String>,
    pub method: Option<Provider>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub redirect_url: Option<String>,
}

/// `FundCheckoutResponse` struct 
/// 
/// **Note**: persist the `id` and the `signature` field in a store if you want to get the details of the fund checkout
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FundCheckoutResponse {
    pub paid: bool,
    pub id: String,
    pub url: String,
    pub amount: Decimal,
    pub signature: String,
    pub currency: Currency,
    pub email: Option<String>,
    pub method: Option<Provider>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub redirect_url: Option<String>,
}
