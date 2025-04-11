#![allow(unused)]
#![allow(unused_imports)]

use anyhow::Result;
use reqwest::{Client, Error as ReqwestErr};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use std::fmt::{self, Debug};
use thiserror::Error as ThisErr;

pub(crate) mod checkout;
pub(crate) mod collection;
pub(crate) mod payment_links;
pub(crate) mod payouts;
pub(crate) mod refunds;
pub(crate) mod wallets;

use checkout::CheckoutsAPI;
use collection::CollectionsAPI;
use payment_links::PaymentLinksAPI;
use payouts::PayoutsAPI;
use refunds::RefundsAPI;
use wallets::WalletsAPI;

/// **[IntaSend](https://intasend.com)** - The _Unofficial_ Rust Client SDK for the Intasend API Gateway.
///
/// This library is a wrapper around the IntaSend Payment Gateway that supports a
/// variety of payment methods e.g Visa, Mastercard, M-Pesa, and even Bitcoin.
///
/// The library is fully async and it uses Reqwest library under the hood to make asynchronous calls to the REST API.
///
/// To use the library you should acquire test or production API keys here: [Sandbox](https://sandbox.intasend.com) or [Production](https://payment.intasend.com)
/// Features include:
///  - `client`: Functionality for applications that need to initiate payments in browser environments
/// - `server`: Functionality for applications that need to receive payments and manage wallets in server environments
/// - `full`: Enables all functionality (both client and server)
///
/// By default, the `full` feature is exposed.

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
    /// println!("[#] Intasend instance: {:#?}", intasend);
    /// ```
    pub fn new(publishable_key: String, secret_key: String, test_mode: bool) -> Self {
        Self {
            publishable_key,
            secret_key,
            test_mode,
        }
    }

    /// The `collection` method returns an instance of the `CollectionsAPI` struct
    ///
    /// ```rust
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
    /// // Collection
    /// let collection: intasend::CollectionsAPI = intasend.collection();
    /// println!("Collection instance: {:#?}", collection);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn collection(&self) -> CollectionsAPI {
        CollectionsAPI {
            intasend: self.clone(),
        }
    }

    /// The `checkout` method returns an instance of the `CheckoutsAPI` struct
    ///
    /// ```rust
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
    /// // Checkout
    /// let checkout: intasend::CheckoutsAPI = intasend.checkout();
    /// println!("Checkout instance: {:#?}", checkout);
    ///
    /// ```
    #[cfg(any(feature = "client", feature = "server"))]
    pub fn checkout(&self) -> CheckoutsAPI {
        CheckoutsAPI {
            intasend: self.clone(),
        }
    }

    /// The `payouts` method returns an instance of the `PayoutsAPI` struct
    ///
    /// ```rust
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
    /// // Payouts
    /// let payouts_api: intasend::PayoutsAPI = intasend.payouts();
    /// println!("Payouts instance: {:#?}", payouts_api);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn payouts(&self) -> PayoutsAPI {
        PayoutsAPI {
            intasend: self.clone(),
        }
    }

    /// The `refunds` method returns an instance of the `RefundsAPI` struct
    ///
    /// ```rust
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
    /// // Refunds
    /// let refunds: intasend::RefundsAPI = intasend.refunds();
    /// println!("Refunds instance: {:#?}", refunds);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn refunds(&self) -> RefundsAPI {
        RefundsAPI {
            intasend: self.clone(),
        }
    }

    /// The `wallets` method returns an instance of the `WalletsAPI` struct
    ///
    /// ```rust
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
    /// // Wallets
    /// let wallets: intasend::WalletsAPI = intasend.wallets();
    /// println!("Wallet instance: {:#?}", wallets);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn wallets(&self) -> WalletsAPI {
        WalletsAPI {
            intasend: self.clone(),
        }
    }

    /// The `payment_links` method returns an instance of the `Wallets` struct
    ///
    /// ```rust
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
    /// // Payment Links
    /// let payment_links: intasend::PaymentLinksAPI = intasend.payment_links();
    /// println!("Payment Links instance: {:#?}", payment_links);
    ///
    /// ```
    #[cfg(feature = "server")]
    pub fn payment_links(&self) -> PaymentLinksAPI {
        PaymentLinksAPI {
            intasend: self.clone(),
        }
    }
}

impl RequestClient for Intasend {
    async fn send_client_request<T, U>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, IntasendClientError>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de> + Debug,
    {
        let client = Client::new();

        let base_url = if self.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        match request_method {
            RequestMethods::Get => {
                let response = client
                    .get(format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .send()
                    .await?;
                // println!("[#] API Response: {:#?}", response);

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    let status = response.status();
                    let error_response = response.json::<IntasendApiError>().await?;
                    Err(IntasendClientError::UnexpectedResponseStatus {
                        status,
                        error: error_response,
                    })
                }
            }
            RequestMethods::Post => {
                let response = client
                    .post(format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await?;
                // println!("[#] API Response: {:#?}", response);

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API parsed Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    let status = response.status();
                    let error_response = response.json::<IntasendApiError>().await?;
                    Err(IntasendClientError::UnexpectedResponseStatus {
                        status,
                        error: error_response,
                    })
                }
            }
            RequestMethods::Put => {
                let response = client
                    .put(format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await?;
                // println!("[#] API Response: {:#?}", response);

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API parsed Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    let status = response.status();
                    let error_response = response.json::<IntasendApiError>().await?;
                    Err(IntasendClientError::UnexpectedResponseStatus {
                        status,
                        error: error_response,
                    })
                }
            }
        }
    }

    async fn send<T, U>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, IntasendClientError>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de> + Debug,
    {
        let client = Client::new();

        let base_url = if self.test_mode {
            "https://sandbox.intasend.com"
        } else {
            "https://payment.intasend.com"
        };

        match request_method {
            RequestMethods::Get => {
                let response = client
                    .get(format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    // .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .send()
                    .await?;
                // .json()
                // .await?;
                // println!("[#] API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                // let json = serde_json::from_value::<U>(response?.json().await?)
                //     .expect("[!] Error parsing json!");

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    let status = response.status();
                    let error_response = response.json::<IntasendApiError>().await?;
                    Err(IntasendClientError::UnexpectedResponseStatus {
                        status,
                        error: error_response,
                    })
                }

                // Ok(response)
            }
            RequestMethods::Post => {
                let response = client
                    .post(format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    // .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await?;

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    let status = response.status();
                    let error_response = response.json::<IntasendApiError>().await?;
                    Err(IntasendClientError::UnexpectedResponseStatus {
                        status,
                        error: error_response,
                    })
                }
            }
            RequestMethods::Put => {
                let response = client
                    .put(format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    .json(&payload)
                    .send()
                    .await?;

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    Ok(parsed_response)
                } else {
                    let status = response.status();
                    let error_response = response.json::<IntasendApiError>().await?;
                    Err(IntasendClientError::UnexpectedResponseStatus {
                        status,
                        error: error_response,
                    })
                }
            }
        }
    }
}

pub trait RequestClient {
    async fn send_client_request<T, U>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, IntasendClientError>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de> + Debug;
    async fn send<T, U>(
        &self,
        payload: Option<T>,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, IntasendClientError>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de> + Debug;
}

// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] io::Error),
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader {
//         expected: String,
//         found: String,
//     },
//     #[error("unknown data store error")]
//     Unknown,
// }

/// `ApiErrorDatail` struct
#[derive(Debug, Deserialize)]
pub struct IntasendApiErrorDetail {
    pub code: String,
    pub detail: String,
    pub attr: Option<String>,
}

/// `IntasendApiError` struct
#[derive(Debug, Deserialize)]
pub struct IntasendApiError {
    pub r#type: String,
    pub errors: Vec<IntasendApiErrorDetail>,
}

/// `IntasendClientError` - The main error data structure derived from `thiserror` crate
#[derive(ThisErr, Debug)]
pub enum IntasendClientError {
    #[error("Intasend client error")]
    ReqwestError(#[from] ReqwestErr),
    #[error("Intasend JSON error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Unexpected response status: {status}\n\nError type: {0}\nDetails: {error:#?}", error.r#type)]
    UnexpectedResponseStatus {
        status: reqwest::StatusCode,
        error: IntasendApiError,
    },
    // #[error("Unexpected response status: {status}")]
    // UnexpectedResponseStatus {
    //     status: reqwest::StatusCode,
    //     #[source]
    //     error: IntasendApiError,
    // },
    // // ... other error variants
}

// impl From<reqwest::Error> for IntasendClientError {
//     fn from(err: reqwest::Error) -> Self {
//         IntasendClientError::ReqwestError(err)
//     }
// }

// impl From<serde_json::Error> for IntasendClientError {
//     fn from(err: serde_json::Error) -> Self {
//         IntasendClientError::SerdeJsonError(err)
//     }
// }

/// `Invoice` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: String,
    pub state: String,
    pub provider: String,
    pub charges: Decimal,
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

/// `Customer` struct
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

/// `CardInfo` struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardInfo {
    pub bin_country: Option<String>,
    pub card_type: Option<String>,
}

/// `Transaction` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub invoice: Option<Invoice>,
    // pub amount: Decimal,
    pub currency: Currency,
    pub value: Decimal,
    pub running_balance: Decimal,
    pub narrative: Option<String>,
    pub trans_type: TransactionType,
    pub status: TransactionStatus,
    pub created_at: String,
    pub updated_at: String,
}

/// `TransactionType` enum
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
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

/// `TransactionStatus` enum
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionStatus {
    Available,
    Clearing,
    #[serde(rename = "ON-HOLD")]
    OnHold,
    Cancelled,
    #[serde(rename = "CHARGEBACK-PENDING")]
    ChargebackPending,
    Refunded,
    Adjustment,
}

/// `Provider` enum - provided by IntaSend
/// Checkout Options supported by Intasend API Gateway
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Provider {
    Mpesa,
    #[serde(rename = "CARD-PAYMENT")]
    CardPayment,
    Bitcoin,
    #[serde(rename = "BANK-ACH")]
    Bank,
    #[serde(rename = "COOP_B2B")]
    CoopB2b,
}

/// `RequestMethods` enum
pub enum RequestMethods {
    Get,
    Post,
    Put,
}

/// Currencies supported by Intasend API Gateway
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    /// Kenya Shillings
    #[serde(rename = "KES")]
    Kes,
    /// US Dollars
    #[serde(rename = "USD")]
    Usd,
    /// Euros
    #[serde(rename = "EUR")]
    Eur,
    /// British Pounds
    #[serde(rename = "GBP")]
    Gbp,
}

/// Tarrifs supported by IntaSend
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Tarrif {
    BusinessPays,
    CustomerPays,
}

/// Payout Provider Options supported by Intasend API Gateway
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum PayoutProvider {
    MpesaB2c,
    MpesaB2b,
    #[serde(rename = "PESALINK")]
    Pesalink,
    #[serde(rename = "INTASEND")]
    Intasend,
    #[serde(rename = "AIRTIME")]
    Airtime,
}
