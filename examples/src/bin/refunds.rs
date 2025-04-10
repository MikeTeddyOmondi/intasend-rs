#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{Intasend, Refund, RefundReason, RefundRequest};

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

    /// RefundsAPI
    let refunds_api = intasend.refunds();
    println!("[#] RefundsAPI instance: {:#?}", refunds_api);

    let refund_request = RefundRequest {
        amount: Decimal::new(1000, 2),
        invoice: "YVB845R".to_string(),
        reason: RefundReason::UnavailableService,
        reason_details: "The service was unavailable".to_string(),
    };

    let refund_list = refunds_api.list().await?;
    println!("[#] Refunds List: {:#?}", refund_list);

    let created_refund: Refund = refunds_api.create(refund_request).await?;
    println!("[#] Created refund: {:?}", created_refund);

    let get_refund = refunds_api.get("JRO97YK".to_string()).await?; // JRO97YK, ERG2PRK
    println!("[#] Refund: {:#?}", get_refund);

    println!("Lib works!");

    Ok(())
}
