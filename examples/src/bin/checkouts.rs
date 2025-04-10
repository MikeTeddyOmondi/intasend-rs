#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse, CheckoutsAPI, Currency, Intasend,
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

    /// CheckoutsAPI
    let checkout: CheckoutsAPI = intasend.checkout();
    // println!("[#] Checkout instance: {:#?}", checkout);

    let checkout_req = CheckoutRequest {
        first_name: Some("Joe".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("joe@doe.com".to_string()),
        method: Some(CheckoutMethod::CardPayment),
        amount: Decimal::new(10000, 2), // 100.00
        currency: Currency::Kes,
    };
    println!("[#] Checkout request: {:#?}", checkout_req);
    println!(
        "[#] Checkout request (JSON): {:#?}",
        serde_json::to_value(&checkout_req)
    );

    let checkout_response: CheckoutResponse = checkout.initiate(checkout_req).await?;
    println!("[#] Checkout response: {:#?}", checkout_response);

    println!("[*] Waiting for the checkout details response...");
    tokio::time::sleep(std::time::Duration::from_secs(45)).await;

    // extract field values from checkout_response struct
    let checkout_details_req = CheckoutDetailsRequest {
        checkout_id: checkout_response.id, // "c33e0d25-d290-4cda-8d7e-02ff38544a46".to_string(),
        signature: checkout_response.signature, // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyJjMzNlMGQyNS1kMjkwLTRjZGEtOGQ3ZS0wMmZmMzg1NDRhNDYiXSwiaWF0IjoxNzIyMzUzMzU1LCJleHAiOjE3MjIzNTY5NTUsImFjY291bnRJRCI6IlpRTTBNOFkiLCJyZWZlcmVuY2UiOiJjMzNlMGQyNS1kMjkwLTRjZGEtOGQ3ZS0wMmZmMzg1NDRhNDYifQ.dZMFD-2k8OnnvvsF8oLEZvEprU6oiBPfCdv0M7tKdnI".to_string(),
    };

    let checkout_details_response: CheckoutDetailsResponse =
        checkout.details(checkout_details_req).await?;
    println!(
        "[#] Checkout details response: {:#?}",
        checkout_details_response
    );

    println!("Lib works!");

    Ok(())
}
