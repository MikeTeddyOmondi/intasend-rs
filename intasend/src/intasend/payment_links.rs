#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use uuid::Uuid;

use crate::Intasend;

use super::{Currency, Customer, Invoice, RequestClient, RequestMethods, Tarrif};

/// `PaymentLinksAPI` struct implements methods for facilitating:
/// 1. Listing payment links
/// 2. Querying details of single payment links
/// 3. Creating new payment links
/// 4. Update details of single payment links
/// 
/// ```rust
/// // Load .env file
/// use dotenvy;
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
/// // PaymentLinksAPI API
/// let payment_links_api: intasend::PaymentLinksAPI = intasend.payment_links();
/// ```
///
#[derive(Debug)]
pub struct PaymentLinksAPI {
    pub(crate) intasend: Intasend,
}

impl PaymentLinksAPI {
    /// The `list` (PaymentLinks API) enables you to list all payment links created.
    ///
    /// ```rust
    /// // PaymentLinksAPI
    /// let payment_links: PaymentLinksAPI = intasend.payment_links();
    ///
    /// let payment_links_list: PaymentLinksListResponse = payment_links.list().await?;
    /// println!("[#] Payment Links List Info: {:#?}", payment_links_list);
    /// ```
    ///
    pub async fn list(&self) -> Result<PaymentLinksListResponse> {
        let service_path: &str = "/api/v1/paymentlinks/";
        let request_method: RequestMethods = RequestMethods::Get;

        let payment_links_list = &self
            .intasend
            .send::<PaymentLinksListRequest, PaymentLinksListResponse>(None, service_path, request_method)
            .await?;

        Ok(payment_links_list.clone())
    }

    /// The `details` (Wallets API) enables you to access wallet's details.
    ///
    /// ```rust
    /// // PaymentLinksAPI
    /// let payment_links: PaymentLinksAPI = intasend.payment_links();
    ///
    /// let payment_links_details: PaymentLink = payment_links.details(payment_link_id).await?;
    /// println!("[#] Payment Links Details: {:#?}", payment_links_list);
    /// ```
    ///
    pub async fn details(&self, payment_link_id: String) -> Result<PaymentLink> {
        let service_path: &str = &format!("/api/v1/paymentlinks/{}", payment_link_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let payment_links_details = &self
            .intasend
            .send::<PaymentLinksDetailsRequest, PaymentLink>(None, service_path, request_method)
            .await?;

        Ok(payment_links_details.clone())
    }

    /// The `create` (PaymentLinksAPI) will help you to create payment links.
    ///
    /// ```rust
    /// // PaymentLinksAPI
    /// let payment_links: PaymentLinksAPI = intasend.wallets();
    ///
    /// let payload = PaymentLinksCreateDetails {
    ///     title: "Payment link Title",
    ///     amount: Some(100),
    ///     usage_limit: Some(1),
    ///     is_active: Some(true),
    ///     mobile_tarrif: Some(Tarrif::BusinessPays),
    ///     card_tarrif: Some(Tarrif::BusinessPays),
    ///     currency: Currency::Kes,
    ///     redirect_url: None,
    /// };
    ///
    /// let created_payment_link: PaymentLink = payment_links.create(payload).await?;
    /// println!("[#] Payment Link Created: {:#?}", created_payment_link);
    /// ```
    pub async fn create(&self, payload: PaymentLinksCreateDetails) -> Result<PaymentLink> {
        let service_path: &str = "/api/v1/paymentlinks/";
        let request_method: RequestMethods = RequestMethods::Post;

        let created_payment_link = &self
            .intasend
            .send::<PaymentLinksCreateDetails, PaymentLink>(Some(payload), service_path, request_method)
            .await?;

        Ok(created_payment_link.clone())
    }

    /// The `update` (PaymentLinksAPI) will help you to update details of a payment link(s).
    ///
    /// ```rust
    /// // PaymentLinksAPI
    /// let payment_links: PaymentLinksAPI = intasend.wallets();
    ///
    /// let payload = PaymentLinksUpdateDetails {
    ///     title: "Payment link Title Updated",
    ///     amount: Some(300),
    ///     usage_limit: Some(6),
    ///     is_active: Some(false),
    ///     mobile_tarrif: Some(Tarrif::BusinessPays),
    ///     card_tarrif: Some(Tarrif::BusinessPays),
    ///     currency: Currency::Kes,
    ///     redirect_url: None,
    /// };
    ///
    /// let uid = Uuid::parse_str(&"e4f6126d-b374-4edb-bf17-f9240d24d66e").unwrap(); 
    /// let updated_payment_link: PaymentLink = payment_links.update(uid.to_string(), payload).await?;
    /// println!("[#] Payment Link Updated: {:#?}", updated_payment_link);
    /// ```
    pub async fn update(&self, payment_link_id: String, payload: PaymentLinksUpdateDetails) -> Result<PaymentLink> {
        let service_path: &str = &format!("/api/v1/paymentlinks/{}", payment_link_id);
        let request_method: RequestMethods = RequestMethods::Put;

        let payment_links_details = &self
            .intasend
            .send::<PaymentLinksUpdateDetails, PaymentLink>(None, service_path, request_method)
            .await?;

        Ok(payment_links_details.clone())
    }
}

/// `PaymentLink` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentLink {
    pub id: Uuid,
    pub title: String,
    pub is_active: bool,
    pub redirect_url: Option<String>,
    pub amount: i32,
    pub usage_limit: i32,
    pub qrcode_file: Option<String>,
    pub url: String,
    pub currency: String,
    pub mobile_tarrif: String,
    pub card_tarrif: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// `PaymentLinksListRequest` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentLinksListRequest {}

/// `PaymentLinksListResponse` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentLinksListResponse {
    pub count: usize,
    pub next: Option<usize>,
    pub previous: Option<usize>,
    pub results: Vec<PaymentLink>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentLinksDetailsRequest {
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentLinksCreateDetails {
    pub title: String,
    pub amount: Option<usize>,
    pub usage_limit: Option<usize>,
    pub is_active: Option<bool>,
    pub mobile_tarrif: Option<Tarrif>,
    pub card_tarrif: Option<Tarrif>,
    pub currency: Currency,
    pub redirect_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentLinksUpdateDetails {
    pub title: String,
    pub amount: Option<usize>,
    pub usage_limit: Option<usize>,
    pub is_active: Option<bool>,
    pub mobile_tarrif: Option<Tarrif>,
    pub card_tarrif: Option<Tarrif>,
    pub currency: Currency,
    pub redirect_url: Option<String>,
}