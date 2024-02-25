#![allow(unused)]
#![allow(unused_imports)]

use anyhow::Result;
use reqwest::{Client, Error};
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


/// **[Intasend](https://intasend.com)** - The _Unoffical_ Rust Client SDK for the Intasend API Gateway.
///
/// This library is a wrapper around the Intasend Payment Gateway that supports a 
/// variety of payment methods e.g Visa, Mastercard, M-Pesa, and even Bitcoin.
///
/// The library is fully async and it uses Reqwest library under the hood to make asynchronous calls to the REST API.  
///
/// To use the library you should acquire test API keys here: [Sandbox](https://sandbox.intasend.com)  
///
/// ```rust
/// let intasend = Intasend { 
///     publishable_key: "ISPubKey_test_c1f90113-3dbb-4201-9b88-f1c2d3056e5c".to_string(), 
///     secret_key: "ISSecretKey_test_5527b085-40b6-460a-9c31-25d58a204d20".to_string(), 
///     test_mode: true 
/// };
/// ```

#[derive(Debug, Deserialize)]
pub struct Intasend {
    pub publishable_key: String,
    pub secret_key: String,
    pub test_mode: bool,
}

/// The Intasend Struct implements a number of methods namely: collection, checkout,
/// payouts, refunds and wallets which adheres to the API specifications provided by Intasend 

impl Intasend {
    /// The new method creates a new instance of the Intasend client
    /// ```rust
    /// let intasend = Intasend::new(
    ///    "ISPubKey_test_c1f90113-3dbb-4201-9b88-f1c2d3056e5c".to_string(),
    ///    "ISSecretKey_test_5527b085-40b6-460a-9c31-25d58a204d20".to_string(),
    ///     true,
    /// );
    /// ```
    pub fn new(publishable_key: String, secret_key: String, test_mode: bool) -> Self {
        Self {
            publishable_key,
            secret_key,
            test_mode,
        }
    }

    /// The collection method returns an instance of the Collection struct
    /// 
    /// ```rust
    /// // Collection
    /// let collection: Collection = intasend.collection();
    /// println!("Collection instance: {:#?}", collection);
    /// 
    /// ```
    pub fn collection(&self) -> Collection {
        Collection {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }

    /// The checkout method returns an instance of the Checkout struct
    /// 
    /// ```rust
    /// // Checkout
    /// let checkout: Checkout = intasend.checkout();
    /// println!("Checkout instance: {:#?}", checkout);
    /// 
    /// ```
    pub fn checkout(&self) -> Checkout {
        Checkout {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }

    /// The payout method returns an instance of the Payouts struct
    /// 
    /// ```rust
    /// // Payouts
    /// let payouts: Payouts = intasend.payouts();
    /// println!("Payouts instance: {:#?}", payout);
    /// 
    /// ```
    pub fn payouts(&self) -> Payouts {
        Payouts {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }

    /// The refunds method returns an instance of the Refunds struct
    /// 
    /// ```rust
    /// // Refunds
    /// let refunds: Refunds = intasend.refunds();
    /// println!("Refunds instance: {:#?}", refunds);
    /// 
    /// ```
    pub fn refunds(&self) -> Refunds {
        Refunds {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }

    /// The wallets method returns an instance of the Wallets struct
    /// 
    /// ```rust
    /// // Wallets
    /// let wallets: Wallets = intasend.refunds();
    /// println!("Refunds instance: {:#?}", wallets);
    /// 
    /// ```
    pub fn wallets(&self) -> Wallets {
        Wallets {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }
}

impl<T> RequestClient<T> for Intasend
where
    T: Serialize,
    // U: for<'a> Deserialize<'a> + Debug,
{
    async fn send(
        &self,
        payload: T,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<JSON, Error> {
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

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                let json = serde_json::from_value::<JSON>(response?.json().await?)
                    .expect("Error parsing json!");

                Ok(json)
            }
            RequestMethods::POST => {
                let response = client
                    .post(&format!("{}{}", base_url, service_path))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", self.secret_key))
                    .header("X-IntaSend-Public-API-Key", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await;
                // println!("API Response: {:#?}", response);

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                let json = serde_json::from_value::<JSON>(response?.json().await?)
                    .expect("Error parsing json!");

                Ok(json)
            }
        }
    }
}

pub trait RequestClient<T> {
    async fn send(
        &self,
        payload: T,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<JSON, Error>;
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
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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