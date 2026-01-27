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

/// `SubscriptionsAPI` struct implements methods for facilitating:
/// 1. Create subscription plan
/// 2. List subscription plans
/// 3. Querying subscription plan details
/// 4. Updating subscription plan details
/// 5. Create subscription customers
/// 6. Listing subscriptions customers
/// 7. Querying subscription customers details
/// 8. Updating subscription customers details
/// 9. Create subscriptions (based on a subscription plan ID)
/// 10. Listing subscriptions
/// 11. Querying subscription details
/// 12. Updating subscription details
/// 13. Cancelling subscriptions using subscription ID
/// 14. Querying transactions of a subscriptions ID
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
/// // SubscriptionsAPI API
/// let subscriptions_api: intasend::SubscriptionsAPI = intasend.subscriptions();
/// ```
///
#[derive(Debug)]
pub struct SubscriptionsAPI {
    pub(crate) intasend: Intasend,
}

impl SubscriptionsAPI {
    /// The `create_plan` (Subscriptions API) enables you to create a subscriptions plan.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    ///
    /// let subscriptions_plan: intasend::SubscriptionsPlan = subscriptions.create_plan().await?;
    /// println!("[#] Subscriptions plan: {:#?}", subscriptions_plan);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn create_plan(&self) -> Result<SubscriptionsPlan> {
        let service_path: &str = "/api/v1/subscriptions/";
        let request_method: RequestMethods = RequestMethods::Get;

        let subscriptions_plan = &self
            .intasend
            .send::<SubscriptionsPlanCreateRequest, SubscriptionsPlan>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(subscriptions_plan.clone())
    }

    /// The `list_plans` (Subscriptions API) enables you to list all subscription plans created.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    ///
    /// let subscriptions_plans_list: intasend::SubscriptionsListResponse = subscriptions.list_plans().await?;
    /// println!("[#] Subscriptions Plans List: {:#?}", subscriptions_plans_list);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn list_plans(&self) -> Result<SubscriptionsPlanListResponse> {
        let service_path: &str = "/api/v1/subscriptions/";
        let request_method: RequestMethods = RequestMethods::Get;

        let subscriptions_plan_list = &self
            .intasend
            .send::<SubscriptionsListRequest, SubscriptionsPlanListResponse>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(subscriptions_plan_list.clone())
    }

    /// The `plan_details` (SubscriptionsAPI) enables you to access single subscription plan's details.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    /// let subscription_id = uuid::Uuid::parse_str("0bd8984a-f487-46fb-b7b6-c17f8e87ccc8").unwrap().to_string();
    /// let subscriptions_plan_details: intasend::Subscription = subscriptions.details(subscription_id).await?;
    /// println!("[#] Subscriptions Plan Details: {:#?}", subscriptions_plan_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn plan_details(&self, subscription_id: String) -> Result<SubscriptionsPlan> {
        let service_path: &str = &format!("/api/v1/subscriptions/{}", subscription_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let subscriptions_plan_details = &self
            .intasend
            .send::<SubscriptionsDetailsRequest, SubscriptionsPlan>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(subscriptions_plan_details.clone())
    }

    /// The `update_plan` (Subscriptions API) enables you to list all subscriptions created.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    ///
    /// let updated_subscriptions_plan: intasend::SubscriptionsPlan = subscriptions.update_plan().await?;
    /// println!("[#] Updated Subscriptions plan: {:#?}", updated_subscriptions_plan);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn update_plan(&self) -> Result<SubscriptionsListResponse> {
        let service_path: &str = "/api/v1/subscriptions/";
        let request_method: RequestMethods = RequestMethods::Get;

        let subscriptions_list = &self
            .intasend
            .send::<SubscriptionsListRequest, SubscriptionsListResponse>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(subscriptions_list.clone())
    }

    /// The `create` (SubscriptionsAPI) will help you to create subscriptions.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    ///
    /// let payload = intasend::SubscriptionsCreateDetails {
    ///     title: "Subscription Title".to_string(),
    ///     amount: Some(100),
    ///     usage_limit: Some(1),
    ///     is_active: Some(true),
    ///     mobile_tarrif: Some(intasend::Tarrif::BusinessPays),
    ///     card_tarrif: Some(intasend::Tarrif::BusinessPays),
    ///     currency: intasend::Currency::Kes,
    ///     redirect_url: None,
    /// };
    ///
    /// let created_payment_link: intasend::PaymentLink = payment_links.create(payload).await?;
    /// println!("[#] Payment Link Created: {:#?}", created_payment_link);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn create(&self, payload: SubscriptionsCreateDetails) -> Result<Subscription> {
        let service_path: &str = "/api/v1/subscriptions/";
        let request_method: RequestMethods = RequestMethods::Post;

        let created_subscription = &self
            .intasend
            .send::<SubscriptionsCreateDetails, Subscription>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(created_subscription.clone())
    }

    /// The `details` (SubscriptionsAPI) enables you to access single subscription's details.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    /// let subscription_id = uuid::Uuid::parse_str("0bd8984a-f487-46fb-b7b6-c17f8e87ccc8").unwrap().to_string();
    /// let subscriptions_details: intasend::Subscription = subscriptions.details(subscription_id).await?;
    /// println!("[#] Subscriptions Details: {:#?}", subscriptions_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn details(&self, subscription_id: String) -> Result<Subscription> {
        let service_path: &str = &format!("/api/v1/subscriptions/{}", subscription_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let subscriptions_details = &self
            .intasend
            .send::<SubscriptionsDetailsRequest, Subscription>(None, service_path, request_method)
            .await?;

        Ok(subscriptions_details.clone())
    }
    /// The `update` (SubscriptionsAPI) enables you to update single subscription's details.
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    /// let subscription_id = uuid::Uuid::parse_str("0bd8984a-f487-46fb-b7b6-c17f8e87ccc8").unwrap().to_string();
    /// let subscriptions_details: intasend::Subscription = subscriptions.update(subscription_id).await?;
    /// println!("[#] Subscriptions Details: {:#?}", subscriptions_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn update(&self, subscription_plan_id: String) -> Result<Subscription> {
        let service_path: &str = &format!("/api/v1/subscriptions/{}", subscription_plan_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let subscriptions_details = &self
            .intasend
            .send::<SubscriptionsDetailsRequest, Subscription>(None, service_path, request_method)
            .await?;

        Ok(subscriptions_details.clone())
    }

    /// The `update` (SubscriptionsAPI) will help you to update details of a subscription(s).
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
    /// // SubscriptionsAPI
    /// let subscriptions: intasend::SubscriptionsAPI = intasend.subscriptions();
    ///
    /// let payload = intasend::SubscriptionsUpdateDetails {
    ///     title: "Subscription Title Updated".to_string(),
    ///     amount: Some(300),
    ///     usage_limit: Some(6),
    ///     is_active: Some(false),
    ///     mobile_tarrif: Some(intasend::Tarrif::BusinessPays),
    ///     card_tarrif: Some(intasend::Tarrif::BusinessPays),
    ///     currency: intasend:: Currency::Kes,
    ///     redirect_url: None,
    /// };
    ///
    /// let uid = uuid::Uuid::parse_str("e4f6126d-b374-4edb-bf17-f9240d24d66e").unwrap();
    /// let updated_payment_link: intasend::PaymentLink = payment_links.update(uid.to_string(), payload).await?;
    /// println!("[#] Payment Link Updated: {:#?}", updated_payment_link);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn create_customers(
        &self,
        subscription_id: String,
        payload: SubscriptionsUpdateDetails,
    ) -> Result<Subscription> {
        let service_path: &str = &format!("/api/v1/subscriptions/{}", subscription_id);
        let request_method: RequestMethods = RequestMethods::Put;

        let subscription_details = &self
            .intasend
            .send::<SubscriptionsUpdateDetails, Subscription>(None, service_path, request_method)
            .await?;

        Ok(subscription_details.clone())
    }
}

/// `SubscriptionsPlanCreateRequest` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsPlanCreateRequest {
    pub currency: Currency,
    pub name: String,
    pub frequency: u32,
    pub frequency_unit: String,
    pub billing_cycles: u32,
    pub amount: String,
}

/// `SubscriptionsPlan` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsPlan {
    pub currency: Currency,
    pub name: String,
    pub frequency: u32,
    pub frequency_unit: String,
    pub billing_cycles: u32,
    pub amount: String,
}

/// `SubscriptionsPlanListResponse` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsPlanListResponse {
    pub count: usize,
    pub next: Option<usize>,
    pub previous: Option<usize>,
    pub results: Vec<SubscriptionsPlan>,
}

/// `Subscription` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscription {
    pub subscription_id: Uuid,
    pub reference: String,
    pub status: SubscriptionStatus,
    pub setup_url: String,
    pub redirect_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubscriptionStatus {
    PENDING,
    ACTIVE,
    CANCELED,
    COMPLETE,
    FAILED,
}

/// `SubscriptionsListRequest` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsListRequest {}

/// `SubscriptionsListResponse` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsListResponse {
    pub count: usize,
    pub next: Option<usize>,
    pub previous: Option<usize>,
    pub results: Vec<Subscription>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsDetailsRequest {
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsCreateDetails {
    pub customer_id: String,
    pub reference: String,
    pub plan_id: String,
    pub currency: Currency,
    /// redirect_url: Redirects your customer after successful payment
    pub redirect_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsUpdateDetails {
    pub title: String,
    pub amount: Option<usize>,
    pub usage_limit: Option<usize>,
    pub is_active: Option<bool>,
    pub mobile_tarrif: Option<Tarrif>,
    pub card_tarrif: Option<Tarrif>,
    pub currency: Currency,
    pub redirect_url: Option<String>,
}
