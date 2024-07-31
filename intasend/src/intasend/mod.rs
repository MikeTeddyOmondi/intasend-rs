#![allow(unused)]
#![allow(unused_imports)]

use anyhow::Result;
use reqwest::{Client, Error as ReqwestErr};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use std::fmt::{self, Debug};
use thiserror::Error as ThisErr;

#[cfg(feature = "client")]
#[cfg(feature = "server")]
pub(crate) mod checkout;
pub(crate) mod collection;
pub(crate) mod payouts;
pub(crate) mod refunds;
pub(crate) mod wallets;

use checkout::CheckoutsAPI;
use collection::CollectionsAPI;
use payouts::PayoutsAPI;
use refunds::RefundsAPI;
use wallets::WalletsAPI;

/// **[IntaSend](https://intasend.com)** - The _Unoffical_ Rust Client SDK for the Intasend API Gateway.
///
/// This library is a wrapper around the IntaSend Payment Gateway that supports a
/// variety of payment methods e.g Visa, Mastercard, M-Pesa, and even Bitcoin.
///
/// The library is fully async and it uses Reqwest library under the hood to make asynchronous calls to the REST API.
///
/// To use the library you should acquire test or production API keys here: [Sandbox](https://sandbox.intasend.com) or [Production](https://payment.intasend.com)
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
    pub fn collection(&self) -> CollectionsAPI {
        CollectionsAPI {
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
    pub fn checkout(&self) -> CheckoutsAPI {
        CheckoutsAPI {
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
    pub fn payouts(&self) -> PayoutsAPI {
        PayoutsAPI {
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
    pub fn refunds(&self) -> RefundsAPI {
        RefundsAPI {
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
    pub fn wallets(&self) -> WalletsAPI {
        WalletsAPI {
            intasend: self.clone(),
        }
    }
}

impl RequestClient for Intasend
{
    #[cfg(feature = "client")]
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
                    .get(&format!("{}{}", base_url, service_path))
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
                    Err(IntasendClientError::UnexpectedResponseStatus(
                        response.status(),
                    ))
                }
            }
            RequestMethods::Post => {
                let response = client
                    .post(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await?;
                println!("[#] API Response: {:#?}", response);

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    Err(IntasendClientError::UnexpectedResponseStatus(
                        response.status(),
                    ))
                }
            }
        }
    }

    #[cfg(feature = "server")]
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
                    .get(&format!("{}{}", base_url, service_path))
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
                    Err(IntasendClientError::UnexpectedResponseStatus(
                        response.status(),
                    ))
                }

                // Ok(response)
            }
            RequestMethods::Post => {
                let response = client
                    .post(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    // .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await?;
                // .json()
                // .await?;
                // println!("[#] API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                // let json = serde_json::from_value::<U>(response?.json().await?)
                //     .expect("[!] Error parsing json!");
                // let json = serde_json::from_str::<U>(&response).expect("[!] Error parsing json!");
                // Ok(json)

                // println!("Response status: {:#?}", response.status());

                if response.status().is_success() {
                    let parsed_response = response.json::<U>().await?;
                    // println!("[#] API Response (OK): {:#?}", parsed_response);
                    Ok(parsed_response)
                } else {
                    Err(IntasendClientError::UnexpectedResponseStatus(
                        response.status(),
                    ))
                }

                // Ok(response)
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

#[derive(ThisErr, Debug)]
pub enum IntasendClientError {
    #[error("Intasend client error")]
    ReqwestError(#[from] ReqwestErr),
    #[error("Intasend json error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Unexpected response status: {0}")]
    UnexpectedResponseStatus(reqwest::StatusCode),
    // ... other error variants
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

pub enum RequestMethods {
    Get,
    Post,
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

// impl fmt::Display for Currency {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.as_str())
//     }
// }

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
