#![allow(unused)]
#![allow(unused_imports)]

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
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

impl<T, U> RequestClient<T, U> for Intasend
where
    T: Serialize,
    U: for<'a> Deserialize<'a> + Debug,
{
    async fn send(
        &self,
        payload: T,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, Error> {
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
                    .json(&payload)
                    .send()
                    .await;

                let transfer_response: U = response?.json().await?;
                println!("{:#?}", transfer_response);

                Ok(transfer_response)
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

                let transfer_response: U = response?.json().await?;
                println!("{:#?}", transfer_response);

                Ok(transfer_response)
            }
        }
    }
}

pub trait RequestClient<T, U> {
    async fn send(
        &self,
        payload: T,
        service_path: &str,
        request_method: RequestMethods,
    ) -> Result<U, Error>;
}

pub enum RequestMethods {
    GET,
    POST,
}
