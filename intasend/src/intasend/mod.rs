#![allow(unused)]
#![allow(unused_imports)]

use anyhow::Result;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use std::fmt::Debug;

pub(crate) mod collection;
pub(crate) mod payouts;
pub(crate) mod refunds;
pub(crate) mod wallets;

use collection::Collection;
use payouts::Payouts;
use refunds::Refunds;
use wallets::Wallets;

#[derive(Debug, Deserialize)]
pub struct Intasend {
    publishable_key: String,
    secret_key: String,
    test_mode: bool,
}

impl Intasend {
    pub fn new(publishable_key: String, secret_key: String, test_mode: bool) -> Self {
        Self {
            publishable_key,
            secret_key,
            test_mode,
        }
    }

    pub fn collection(&self) -> Collection {
        Collection {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }

    pub fn payouts(&self) -> Payouts {
        Payouts {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }

    pub fn refunds(&self) -> Refunds {
        Refunds {
            intasend: Intasend::new(
                self.publishable_key.clone(),
                self.secret_key.clone(),
                self.test_mode,
            ),
        }
    }
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
                    .header("INTASEND_PUBLIC_API_KEY", self.publishable_key.clone())
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
                    .header("INTASEND_PUBLIC_API_KEY", self.publishable_key.clone())
                    .json(&payload)
                    .send()
                    .await;

                // let json: Map<String, Value> = serde_json::from_str(response)?;
                let json = serde_json::from_value::<JSON>(response?.json().await?).expect("Error parsing json!");

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
