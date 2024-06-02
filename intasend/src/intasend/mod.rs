#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Result,Error as StdErr};
use reqwest::{Client, Error};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use std::fmt::Debug;

#[cfg(feature = "client")]
#[cfg(feature = "server")]
pub(crate) mod checkout;
pub(crate) mod collection;
pub(crate) mod payouts;
pub(crate) mod refunds;
pub(crate) mod wallets;

use checkout::Checkout;
use collection::Collection;
use payouts::Payouts;
use refunds::Refunds;
use wallets::Wallets;

/// **[IntaSend](https://intasend.com)** - The _Unoffical_ Rust Client SDK for the Intasend API Gateway.
///
/// This library is a wrapper around the IntaSend Payment Gateway that supports a
/// variety of payment methods e.g Visa, Mastercard, M-Pesa, and even Bitcoin.
///
/// The library is fully async and it uses Reqwest library under the hood to make asynchronous calls to the REST API.  
///
/// To use the library you should acquire test API keys here: [Sandbox](https://sandbox.intasend.com)  
///

#[derive(Clone, Debug, Deserialize)]
pub struct Intasend {
    publishable_key: String,
    secret_key: String,
    test_mode: bool,
}

/// The Intasend Struct implements a number of methods namely: collection, checkout,
/// payouts, refunds and wallets which adheres to the API specifications provided by Intasend

impl Intasend {
    /// The `new` method creates a new instance of the `Intasend` client
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
    /// println!("[#] Intasend instance: {:#?}", intasend);
    /// ```
    pub fn new(publishable_key: String, secret_key: String, test_mode: bool) -> Self {
        Self {
            publishable_key,
            secret_key,
            test_mode,
        }
    }

    /// The `collection` method returns an instance of the `Collection` struct
    ///
    /// ```rust
    /// // Collection
    /// let collection: Collection = intasend.collection();
    /// println!("Collection instance: {:#?}", collection);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn collection(&self) -> Collection {
        Collection {
            intasend: self.clone(),
        }
    }

    /// The `checkout` method returns an instance of the `Checkout` struct
    ///
    /// ```rust
    /// // Checkout
    /// let checkout: Checkout = intasend.checkout();
    /// println!("Checkout instance: {:#?}", checkout);
    ///
    /// ```
    #[cfg(feature = "client")]
    pub fn checkout(&self) -> Checkout {
        Checkout {
            intasend: self.clone(),
        }
    }

    /// The `payouts` method returns an instance of the `Payouts` struct
    ///
    /// ```rust
    /// // Payouts
    /// let payouts: Payouts = intasend.payouts();
    /// println!("Payouts instance: {:#?}", payout);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn payouts(&self) -> Payouts {
        Payouts {
            intasend: self.clone(),
        }
    }

    /// The `refunds` method returns an instance of the `Refunds` struct
    ///
    /// ```rust
    /// // Refunds
    /// let refunds: Refunds = intasend.refunds();
    /// println!("Refunds instance: {:#?}", refunds);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn refunds(&self) -> Refunds {
        Refunds {
            intasend: self.clone(),
        }
    }

    /// The `wallets` method returns an instance of the `Wallets` struct
    ///
    /// ```rust
    /// // Wallets
    /// let wallets: Wallets = intasend.refunds();
    /// println!("Refunds instance: {:#?}", wallets);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn wallets(&self) -> Wallets {
        Wallets {
            intasend: self.clone(),
        }
    }
}

impl RequestClient for Intasend
// where
//     T: Serialize,
//     U: for<'de> Deserialize<'de> + Debug,
{
    #[cfg(feature = "client")]
    async fn send_client_request<T: Serialize, U: for<'de> Deserialize<'de> + Debug>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, StdErr> {
        let client = Client::new();

        let base_url = if self.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        match request_method {
            RequestMethods::GET => {
                let response = client
                    .get(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    // .header("Authorization", format!("Bearer {}", self.secret_key))
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .send()
                    .await;
                // println!("[#] API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                let json = serde_json::from_value::<U>(response?.json().await?)
                    .expect("[!] Error parsing json!");

                Ok(json)
            }
            RequestMethods::POST => {
                let response = client
                    .post(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    // .header("Authorization", format!("Bearer {}", self.secret_key))
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await;
                println!("[#] API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                let json = serde_json::from_value::<U>(response?.json().await?)
                    .expect("[!] Error parsing json!");

                Ok(json)
            }
        }
    }

    #[cfg(feature = "server")]
    async fn send<T: Serialize, U: for<'de> Deserialize<'de> + Debug>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, StdErr> {
        let client = Client::new();

        let base_url = if self.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        match request_method {
            RequestMethods::GET => {
                let response = client
                    .get(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .send()
                    .await;
                // println!("[#] API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                let json = serde_json::from_value::<U>(response?.json().await?)
                    .expect("[!] Error parsing json!");

                Ok(json)
            }
            RequestMethods::POST => {
                let response: U = client
                    .post(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    // .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await?
                    .json()
                    .await?;
                println!("[#] API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                // let json = serde_json::from_value::<U>(response?.json().await?)
                //     .expect("[!] Error parsing json!");
                // let json = serde_json::from_str::<U>(&response).expect("[!] Error parsing json!");

                // Ok(json)
                Ok(response)
            }
        }
    }
}

pub trait RequestClient {
    async fn send_client_request<T: Serialize, U: for<'de> Deserialize<'de> + Debug>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, StdErr>;
    async fn send<T: Serialize, U: for<'de> Deserialize<'de> + Debug>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, StdErr>;
}

#[derive(Debug)]
pub enum RequestClientError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    // ... other error variants
}

impl From<reqwest::Error> for RequestClientError {
    fn from(err: reqwest::Error) -> Self {
        RequestClientError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for RequestClientError {
    fn from(err: serde_json::Error) -> Self {
        RequestClientError::SerdeJsonError(err)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: String,
    pub state: String,
    pub provider: String,
    pub charges: String,
    pub net_amount: Decimal,
    pub currency: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardInfo {
    pub bin_country: Option<String>,
    pub card_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub amount: Decimal,
    pub currency: String,
    pub value: String,
    pub running_balance: String,
    pub narrative: String,
    pub trans_type: TransactionType,
    pub status: TransactionStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionType {
    Sale,
    Adjustment,
    Payout,
    Charge,
    Airtime,
    Deposit,
    Exchange,
    Unmarked,
}

impl TransactionType {
    fn as_str(&self) -> String {
        match self {
            TransactionType::Sale => "SALE".to_string(),
            TransactionType::Adjustment => "ADJUSTMENT".to_string(),
            TransactionType::Payout => "PAYOUT".to_string(),
            TransactionType::Charge => "CHARGE".to_string(),
            TransactionType::Airtime => "AIRTIME".to_string(),
            TransactionType::Deposit => "DEPOSIT".to_string(),
            TransactionType::Exchange => "EXCHANGE".to_string(),
            TransactionType::Unmarked => "UNMARKED".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionStatus {
    Available,
    Clearing,
    OnHold,
    Cancelled,
    ChargebackPending,
    Refunded,
    Adjustment,
}

impl TransactionStatus {
    fn as_str(&self) -> String {
        match self {
            TransactionStatus::Available => "AVAILABLE".to_string(),
            TransactionStatus::Clearing => "CLEARING".to_string(),
            TransactionStatus::OnHold => "ON-HOLD".to_string(),
            TransactionStatus::Cancelled => "CANCELLED".to_string(),
            TransactionStatus::ChargebackPending => "CHARGEBACK-PENDING".to_string(),
            TransactionStatus::Refunded => "REFUNDED".to_string(),
            TransactionStatus::Adjustment => "ADJUSTMENT".to_string(),
        }
    }
}

/// Checkout Options supported by Intasend API Gateway
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Provider {
    Mpesa,
    CardPayment,
    Bitcoin,
    Bank,
    CoopB2b,
}

impl Provider {
    pub fn as_str(&self) -> String {
        match self {
            Provider::Mpesa => "MPESA".to_string(),
            Provider::CardPayment => "CARD-PAYMENT".to_string(),
            Provider::Bitcoin => "BITCOIN".to_string(),
            Provider::Bank => "BANK-ACH".to_string(),
            Provider::CoopB2b => "COOP_B2B".to_string(),
        }
    }
}

pub enum RequestMethods {
    GET,
    POST,
}

// Define the trait for JSON conversion
trait FromJsonValue {
    fn from_value(value: &JSON) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

/// Currencies supported by Intasend API Gateway
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Currency {
    /// Kenya Shillings
    KES,
    /// US Dollars
    USD,
    /// Euros
    EUR,
    /// British Pounds
    GBP,
}

impl Currency {
    fn as_str(&self) -> String {
        match self {
            Currency::KES => "KES".to_string(),
            Currency::USD => "USD".to_string(),
            Currency::EUR => "EUR".to_string(),
            Currency::GBP => "GBP".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Tarrif {
    BusinessPays,
    CustomerPays,
}

impl Tarrif {
    pub fn as_str(&self) -> String {
        match self {
            Tarrif::BusinessPays => "BUSINESS-PAYS".to_string(),
            Tarrif::CustomerPays => "CUSTOMER-PAYS".to_string(),
        }
    }
    pub fn from_str(s: String) -> Option<Self> {
        match s.as_str() {
            "BUSINESS-PAYS" => Some(Tarrif::BusinessPays),
            "CUSTOMER-PAYS" => Some(Tarrif::CustomerPays),
            _ => None, // Return None if the string doesn't match any variant
        }
    }
}
