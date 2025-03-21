#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{CollectionsAPI, Intasend, MpesaStkPushRequest, MpesaStkPushResponse, StkPushStatusRequest};

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

    /// CollectionsAPI
    let collections_api: CollectionsAPI = intasend.collection();
    println!("[#] CollectionsAPI instance: {:#?}", collections_api);

    let stkpush_request = MpesaStkPushRequest {
        amount: Decimal::new(1000, 2), // "10.00".to_string(),
        phone_number: "254717135176".to_string(),
        api_ref: None,
        wallet_id: None,
    };

    let stkpush_response: MpesaStkPushResponse =
        collections_api.mpesa_stk_push(stkpush_request).await?;
    println!("[#] Mpesa STK push: {:#?}", stkpush_response);

    println!("[*] Waiting for the collection response...");
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    let stkpushstatus_req = StkPushStatusRequest {
        invoice_id: stkpush_response.invoice.unwrap().invoice_id, // "RXX5P8R".to_string(),
        checkout_id: None,
        signature: None,
    };

    let stkpushstatus = collections_api.status(stkpushstatus_req).await?;
    println!("[#] Mpesa STK Push Status Response: {:#?}", stkpushstatus);

    println!("Lib works!");

    Ok(())
}
