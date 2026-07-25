#![allow(unused)]

use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    Currency, Decimal, FrequencyUnit, Intasend, Subscription, SubscriptionsAPI,
    SubscriptionsCreateDetails, SubscriptionsCustomer, SubscriptionsCustomerCreateDetails,
    SubscriptionsCustomerListResponse, SubscriptionsListResponse, SubscriptionsPlan,
    SubscriptionsPlanCreateRequest, SubscriptionsPlanListResponse,
    SubscriptionsTransactionListResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    let intasend_public_key =
        env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    let intasend_secret_key =
        env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");

    /// Intasend Client
    let intasend = Intasend::new(intasend_public_key, intasend_secret_key, true);
    // println!("[#] Intasend instance: {:#?}", intasend);

    /// SubscriptionsAPI
    let subscriptions: SubscriptionsAPI = intasend.subscriptions();
    println!("[#] Subscriptions instance: {:#?}", subscriptions);

    // -------------------------------------------------------------------------
    // Subscription Plans
    // -------------------------------------------------------------------------

    let plan_request = SubscriptionsPlanCreateRequest {
        name: "Netflix Monthly".to_string(),
        frequency: 1,
        frequency_unit: FrequencyUnit::Month,
        billing_cycles: 12,
        currency: Currency::Kes,
        amount: Decimal::from(1000),
        reference: None,
        redirect_url: None,
    };

    let created_plan: SubscriptionsPlan = subscriptions.create_plan(plan_request).await?;
    println!("[#] Subscription Plan Created: {:#?}", created_plan);

    let plans_list: SubscriptionsPlanListResponse = subscriptions.list_plans().await?;
    println!("[#] Subscription Plans List: {:#?}", plans_list);

    // The `plan_id` returned by `create_plan` identifies the plan for detail / update calls.
    let plan_id = created_plan.plan_id.clone();

    let plan_details: SubscriptionsPlan = subscriptions.plan_details(plan_id.clone()).await?;
    println!("[#] Subscription Plan Details: {:#?}", plan_details);

    let updated_plan_request = SubscriptionsPlanCreateRequest {
        name: "Netflix Monthly Updated".to_string(),
        frequency: 1,
        frequency_unit: FrequencyUnit::Month,
        billing_cycles: 24,
        currency: Currency::Kes,
        amount: Decimal::from(1500),
        reference: None,
        redirect_url: None,
    };

    let updated_plan: SubscriptionsPlan = subscriptions
        .update_plan(plan_id.clone(), updated_plan_request)
        .await?;
    println!("[#] Subscription Plan Updated: {:#?}", updated_plan);

    // -------------------------------------------------------------------------
    // Subscription Customers
    // -------------------------------------------------------------------------

    let customer_request = SubscriptionsCustomerCreateDetails {
        email: "hi@mt0.dev".to_string(),
        first_name: "mt0".to_string(),
        last_name: "Dev".to_string(),
        reference: None,
        address: None,
        city: None,
        state: None,
        zipcode: None,
        country: Some("KE".to_string()),
    };

    let created_customer: SubscriptionsCustomer =
        subscriptions.create_customers(customer_request).await?;
    println!("[#] Subscription Customer Created: {:#?}", created_customer);

    let customers_list: SubscriptionsCustomerListResponse = subscriptions.list_customers().await?;
    println!("[#] Subscription Customers List: {:#?}", customers_list);

    let customer_id = created_customer.customer_id.clone();

    let customer_details: SubscriptionsCustomer =
        subscriptions.customer_details(customer_id.clone()).await?;
    println!("[#] Subscription Customer Details: {:#?}", customer_details);

    let updated_customer_request = SubscriptionsCustomerCreateDetails {
        email: "hi@mt0.dev".to_string(),
        first_name: "mt0".to_string(),
        last_name: "Devv".to_string(),
        reference: None,
        address: None,
        city: None,
        state: None,
        zipcode: None,
        country: Some("KE".to_string()),
    };

    let updated_customer: SubscriptionsCustomer = subscriptions
        .update_customer(customer_id.clone(), updated_customer_request)
        .await?;
    println!("[#] Subscription Customer Updated: {:#?}", updated_customer);

    // -------------------------------------------------------------------------
    // Subscriptions
    // -------------------------------------------------------------------------

    let subscription_request = SubscriptionsCreateDetails {
        customer_id: customer_id.clone(),
        reference: "sub-ref-001".to_string(),
        plan_id: plan_id.clone(),
        currency: Currency::Kes,
        redirect_url: Some("https://example.com/return".to_string()),
    };

    let created_subscription: Subscription = subscriptions.create(subscription_request).await?;
    println!("[#] Subscription Created: {:#?}", created_subscription);

    let subscriptions_list: SubscriptionsListResponse = subscriptions.list().await?;
    println!("[#] Subscriptions List: {:#?}", subscriptions_list);

    let subscription_id = created_subscription.subscription_id.to_string();

    let subscription_details: Subscription =
        subscriptions.details(subscription_id.clone()).await?;
    println!("[#] Subscription Details: {:#?}", subscription_details);

    let update_subscription_request = SubscriptionsCreateDetails {
        customer_id: customer_id.clone(),
        reference: "sub-ref-001".to_string(),
        plan_id: plan_id.clone(),
        currency: Currency::Kes,
        redirect_url: Some("https://example.com/return".to_string()),
    };

    let updated_subscription: Subscription = subscriptions
        .update(subscription_id.clone(), update_subscription_request)
        .await?;
    println!("[#] Subscription Updated: {:#?}", updated_subscription);

    let subscription_transactions: SubscriptionsTransactionListResponse =
        subscriptions.transactions(subscription_id.clone()).await?;
    println!(
        "[#] Subscription Transactions: {:#?}",
        subscription_transactions
    );

    let unsubscribed: Subscription = subscriptions.unsubscribe(subscription_id.clone()).await?;
    println!("[#] Unsubscribed: {:#?}", unsubscribed);

    println!("Lib works!");

    Ok(())
}
