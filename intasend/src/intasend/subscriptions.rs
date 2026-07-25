#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSON;
use uuid::Uuid;

use crate::Intasend;

use super::{Currency, RequestClient, RequestMethods, Transaction};

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
    /// The `create_plan` (Subscriptions API) enables you to create a subscription plan.
    /// POST /https://api.intasend.com/api/v1/subscriptions-plans/
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
    /// let payload = intasend::SubscriptionsPlanCreateRequest {
    ///     currency: intasend::Currency::Kes,
    ///     name: "Premium Monthly".to_string(),
    ///     frequency: 1,
    ///     frequency_unit: intasend::FrequencyUnit::Month,
    ///     billing_cycles: 12,
    ///     amount: intasend::Decimal::from(1000),
    ///     reference: None,
    ///     redirect_url: None,
    /// };
    ///
    /// let subscriptions_plan: intasend::SubscriptionsPlan = subscriptions.create_plan(payload).await?;
    /// println!("[#] Subscriptions plan: {:#?}", subscriptions_plan);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn create_plan(
        &self,
        payload: SubscriptionsPlanCreateRequest,
    ) -> Result<SubscriptionsPlan> {
        let service_path: &str = "/api/v1/subscriptions-plans/";
        let request_method: RequestMethods = RequestMethods::Post;

        let subscriptions_plan = &self
            .intasend
            .send::<SubscriptionsPlanCreateRequest, SubscriptionsPlan>(
                Some(payload),
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
    /// let subscriptions_plans_list: intasend::SubscriptionsPlanListResponse = subscriptions.list_plans().await?;
    /// println!("[#] Subscriptions Plans List: {:#?}", subscriptions_plans_list);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn list_plans(&self) -> Result<SubscriptionsPlanListResponse> {
        let service_path: &str = "/api/v1/subscriptions-plans/";
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
    /// // `plan_id` comes from a `create_plan` / `list_plans` response.
    /// let plan_id = "your-plan-id".to_string();
    /// let subscriptions_plan_details: intasend::SubscriptionsPlan = subscriptions.plan_details(plan_id).await?;
    /// println!("[#] Subscriptions Plan Details: {:#?}", subscriptions_plan_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn plan_details(&self, plan_id: String) -> Result<SubscriptionsPlan> {
        let service_path: &str = &format!("/api/v1/subscriptions-plans/{}/", plan_id);
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

    /// The `update_plan` (Subscriptions API) enables you to update a subscription plan's details.
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
    /// // `plan_id` comes from a `create_plan` / `list_plans` response.
    /// let plan_id = "your-plan-id".to_string();
    /// let payload = intasend::SubscriptionsPlanCreateRequest {
    ///     currency: intasend::Currency::Kes,
    ///     name: "Premium Monthly Updated".to_string(),
    ///     frequency: 1,
    ///     frequency_unit: intasend::FrequencyUnit::Month,
    ///     billing_cycles: 24,
    ///     amount: intasend::Decimal::from(1500),
    ///     reference: None,
    ///     redirect_url: None,
    /// };
    ///
    /// let updated_subscriptions_plan: intasend::SubscriptionsPlan = subscriptions.update_plan(plan_id, payload).await?;
    /// println!("[#] Updated Subscriptions plan: {:#?}", updated_subscriptions_plan);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn update_plan(
        &self,
        plan_id: String,
        payload: SubscriptionsPlanCreateRequest,
    ) -> Result<SubscriptionsPlan> {
        let service_path: &str = &format!("/api/v1/subscriptions-plans/{}/", plan_id);
        let request_method: RequestMethods = RequestMethods::Put;

        let updated_subscriptions_plan = &self
            .intasend
            .send::<SubscriptionsPlanCreateRequest, SubscriptionsPlan>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(updated_subscriptions_plan.clone())
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
    ///     customer_id: "cus_123".to_string(),
    ///     reference: "sub-ref-001".to_string(),
    ///     plan_id: "plan_123".to_string(),
    ///     currency: intasend::Currency::Kes,
    ///     redirect_url: None,
    /// };
    ///
    /// let created_subscription: intasend::Subscription = subscriptions.create(payload).await?;
    /// println!("[#] Subscription Created: {:#?}", created_subscription);
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

    /// The `list` (SubscriptionsAPI) will help you to list subscriptions.
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
    /// let subscriptions_list: intasend::SubscriptionsListResponse = subscriptions.list().await?;
    /// println!("[#] Subscriptions List: {:#?}", subscriptions_list);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<SubscriptionsListResponse> {
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
        let service_path: &str = &format!("/api/v1/subscriptions/{}/", subscription_id);
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
    ///
    /// let payload = intasend::SubscriptionsCreateDetails {
    ///     customer_id: "cus_123".to_string(),
    ///     reference: "sub-ref-001".to_string(),
    ///     plan_id: "plan_123".to_string(),
    ///     currency: intasend::Currency::Kes,
    ///     redirect_url: None,
    /// };
    ///
    /// let subscriptions_details: intasend::Subscription = subscriptions.update(subscription_id, payload).await?;
    /// println!("[#] Subscriptions Details: {:#?}", subscriptions_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn update(
        &self,
        subscription_id: String,
        payload: SubscriptionsCreateDetails,
    ) -> Result<Subscription> {
        let service_path: &str = &format!("/api/v1/subscriptions/{}/", subscription_id);
        let request_method: RequestMethods = RequestMethods::Put;

        let subscriptions_details = &self
            .intasend
            .send::<SubscriptionsCreateDetails, Subscription>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(subscriptions_details.clone())
    }

    /// The `unsubscribe` (SubscriptionsAPI) enables you to unsubscribe a customer from a subscription.
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
    /// let subscriptions_details: intasend::Subscription = subscriptions.unsubscribe(subscription_id).await?;
    /// println!("[#] Subscriptions Details: {:#?}", subscriptions_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn unsubscribe(&self, subscription_id: String) -> Result<Subscription> {
        let service_path: &str =
            &format!("/api/v1/subscriptions/{}/unsubscribe/", subscription_id);
        let request_method: RequestMethods = RequestMethods::Post;

        let subscriptions_details = &self
            .intasend
            .send::<SubscriptionsDetailsRequest, Subscription>(None, service_path, request_method)
            .await?;

        Ok(subscriptions_details.clone())
    }

    /// The `transactions` (SubscriptionsAPI) enables you to get the transactions made by customers under a specific subscription.
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
    /// let subscription_transactions: intasend::SubscriptionsTransactionListResponse = subscriptions.transactions(subscription_id).await?;
    /// println!("[#] Subscription Transactions: {:#?}", subscription_transactions);
    ///
    /// Ok(())
    /// # }
    /// ```
    ///
    pub async fn transactions(
        &self,
        subscription_id: String,
    ) -> Result<SubscriptionsTransactionListResponse> {
        let service_path: &str =
            &format!("/api/v1/subscriptions/{}/transactions/", subscription_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let subscription_transactions = &self
            .intasend
            .send::<SubscriptionsDetailsRequest, SubscriptionsTransactionListResponse>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(subscription_transactions.clone())
    }

    /// The `create_customers` (SubscriptionsAPI) will help you to create customers tied to subscriptions.
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
    /// let payload = intasend::SubscriptionsCustomerCreateDetails {
    ///     email: "customer@example.com".to_string(),
    ///     first_name: "Jane".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     reference: None,
    ///     address: None,
    ///     city: None,
    ///     state: None,
    ///     zipcode: None,
    ///     country: Some("KE".to_string()),
    /// };
    ///
    /// let created_customer: intasend::SubscriptionsCustomer = subscriptions.create_customers(payload).await?;
    /// println!("[#] Subscription Customer Created: {:#?}", created_customer);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn create_customers(
        &self,
        payload: SubscriptionsCustomerCreateDetails,
    ) -> Result<SubscriptionsCustomer> {
        let service_path: &str = "/api/v1/subscriptions-customers/";
        let request_method: RequestMethods = RequestMethods::Post;

        let created_customer = &self
            .intasend
            .send::<SubscriptionsCustomerCreateDetails, SubscriptionsCustomer>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(created_customer.clone())
    }

    /// The `list_customers` (SubscriptionsAPI) will help you to list subscription customers.
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
    /// let customers_list: intasend::SubscriptionsCustomerListResponse = subscriptions.list_customers().await?;
    /// println!("[#] Subscription Customers List: {:#?}", customers_list);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn list_customers(&self) -> Result<SubscriptionsCustomerListResponse> {
        let service_path: &str = "/api/v1/subscriptions-customers/";
        let request_method: RequestMethods = RequestMethods::Get;

        let subscription_customer_list = &self
            .intasend
            .send::<SubscriptionsListRequest, SubscriptionsCustomerListResponse>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(subscription_customer_list.clone())
    }

    /// The `customer_details` (SubscriptionsAPI) will help you to retrieve a customer's details.
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
    /// let customer_id = "cus_123".to_string();
    /// let customer_details: intasend::SubscriptionsCustomer = subscriptions.customer_details(customer_id).await?;
    /// println!("[#] Subscription Customer Details: {:#?}", customer_details);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn customer_details(&self, customer_id: String) -> Result<SubscriptionsCustomer> {
        let service_path: &str = &format!("/api/v1/subscriptions-customers/{}/", customer_id);
        let request_method: RequestMethods = RequestMethods::Get;

        let customer_details = &self
            .intasend
            .send::<SubscriptionsDetailsRequest, SubscriptionsCustomer>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(customer_details.clone())
    }

    /// The `update_customer` (SubscriptionsAPI) will help you to update a customer's details.
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
    /// let customer_id = "cus_123".to_string();
    /// let payload = intasend::SubscriptionsCustomerCreateDetails {
    ///     email: "customer@example.com".to_string(),
    ///     first_name: "Jane".to_string(),
    ///     last_name: "Doe".to_string(),
    ///     reference: None,
    ///     address: None,
    ///     city: None,
    ///     state: None,
    ///     zipcode: None,
    ///     country: Some("KE".to_string()),
    /// };
    ///
    /// let updated_customer: intasend::SubscriptionsCustomer = subscriptions.update_customer(customer_id, payload).await?;
    /// println!("[#] Subscription Customer Updated: {:#?}", updated_customer);
    ///
    /// Ok(())
    /// # }
    /// ```
    pub async fn update_customer(
        &self,
        customer_id: String,
        payload: SubscriptionsCustomerCreateDetails,
    ) -> Result<SubscriptionsCustomer> {
        let service_path: &str = &format!("/api/v1/subscriptions-customers/{}/", customer_id);
        let request_method: RequestMethods = RequestMethods::Put;

        let updated_customer = &self
            .intasend
            .send::<SubscriptionsCustomerCreateDetails, SubscriptionsCustomer>(
                Some(payload),
                service_path,
                request_method,
            )
            .await?;

        Ok(updated_customer.clone())
    }
}

/// Serde adapter for monetary `amount` fields.
///
/// On the wire the IntaSend API documents `amount` as a decimal *string*
/// (`^-?\d{0,13}(?:\.\d{0,2})?$`), but in practice the sandbox echoes it back as a JSON *number*.
/// To be robust we always **serialize as a string** (matching the documented request contract)
/// while **deserializing from either a string or a number**.
mod amount_serde {
    use rust_decimal::Decimal;
    use serde::{de, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S: Serializer>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Decimal, D::Error> {
        struct AmountVisitor;

        impl de::Visitor<'_> for AmountVisitor {
            type Value = Decimal;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a decimal as a string or a number")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Decimal, E> {
                Decimal::from_str(v).map_err(de::Error::custom)
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Decimal, E> {
                Ok(Decimal::from(v))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Decimal, E> {
                Ok(Decimal::from(v))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Decimal, E> {
                Decimal::try_from(v).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_any(AmountVisitor)
    }
}

/// `FrequencyUnit` enum — the billing interval unit for a subscription plan.
///
/// Serializes to the single-character codes the IntaSend API expects:
/// `D` (day), `W` (week), `M` (month), `Y` (year).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FrequencyUnit {
    #[serde(rename = "D")]
    Day,
    #[serde(rename = "W")]
    Week,
    #[serde(rename = "M")]
    Month,
    #[serde(rename = "Y")]
    Year,
}

/// `SubscriptionsPlanCreateRequest` struct — body for creating/updating a subscription plan.
///
/// Mirrors the writable fields of the API `PlanSer` schema; `plan_id`, `plan_url`, `created_at`
/// and `updated_at` are read-only and only appear on [`SubscriptionsPlan`] responses.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsPlanCreateRequest {
    pub name: String,
    pub frequency: u32,
    pub frequency_unit: FrequencyUnit,
    pub billing_cycles: u32,
    pub currency: Currency,
    /// Amount charged per billing cycle. Serialized as a decimal string (e.g. `"1000.00"`) to
    /// match the API contract (`^-?\d{0,13}(?:\.\d{0,2})?$`).
    #[serde(with = "amount_serde")]
    pub amount: Decimal,
    /// Third-party/internal reference ID.
    pub reference: Option<String>,
    /// Redirect your customer after successful payment.
    pub redirect_url: Option<String>,
}

/// `SubscriptionsPlan` struct — the API `PlanSer` response shape.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsPlan {
    pub name: String,
    /// Unique identifier for the plan. Pass this to [`SubscriptionsAPI::plan_details`] /
    /// [`SubscriptionsAPI::update_plan`] to act on a single plan.
    pub plan_id: String,
    pub frequency: u32,
    pub frequency_unit: FrequencyUnit,
    pub billing_cycles: Option<u32>,
    pub currency: Currency,
    #[serde(with = "amount_serde")]
    pub amount: Decimal,
    pub reference: Option<String>,
    pub plan_url: String,
    pub redirect_url: Option<String>,
    // NOTE: `created_at` / `updated_at` are kept as `String` for now, matching the other modules.
    // See BACKLOG.md for the planned crate-wide migration to a typed date/time that won't break
    // backwards compatibility.
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
    /// Identifier for the subscription. A short alphanumeric string (not a UUID).
    pub subscription_id: String,
    pub reference: Option<String>,
    pub status: SubscriptionStatus,
    pub setup_url: String,
    pub redirect_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// `SubscriptionStatus` enum
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubscriptionStatus {
    PENDING,
    ACTIVE,
    CANCELED,
    COMPLETE,
    FAILED,
}

/// `SubscriptionsListRequest` struct — placeholder body for `GET` list endpoints.
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

/// `SubscriptionsDetailsRequest` struct — placeholder body for `GET` detail endpoints.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsDetailsRequest {}

/// `SubscriptionsCreateDetails` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsCreateDetails {
    pub customer_id: String,
    pub reference: String,
    pub plan_id: String,
    pub currency: Currency,
    /// redirect_url: Redirects your customer after successful payment
    pub redirect_url: Option<String>,
}

/// `SubscriptionsCustomerCreateDetails` struct — body for creating/updating a subscription
/// customer.
///
/// Mirrors the writable fields of the API `CustomerSer` schema; `customer_id`, `created_at` and
/// `updated_at` are read-only and only appear on [`SubscriptionsCustomer`] responses.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsCustomerCreateDetails {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    /// Third-party reference ID.
    pub reference: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    /// ISO country code (e.g. `KE`, `GH`, `NG`, `UG`, `TZ`, `CM`, `BF`, `CI`).
    pub country: Option<String>,
}

/// `SubscriptionsCustomer` struct — the API `CustomerSer` response shape.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsCustomer {
    pub customer_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub reference: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub country: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// `SubscriptionsCustomerListResponse` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionsCustomerListResponse {
    pub count: usize,
    pub next: Option<usize>,
    pub previous: Option<usize>,
    pub results: Vec<SubscriptionsCustomer>,
}

/// `SubscriptionsTransactionListResponse` — the subscription transactions endpoint returns a bare
/// JSON array of transactions (not a paginated object), so this is a transparent newtype over the
/// list. Access the inner items via `.0`.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubscriptionsTransactionListResponse(pub Vec<Transaction>);
