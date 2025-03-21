#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    Currency, Intasend, PayoutApproval, PayoutApprovalRequest, PayoutProvider, PayoutRequest,
    PayoutRequestTransaction, PayoutStatusRequest,
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

    /// PayoutsAPI
    let payouts = intasend.payouts();
    println!("[#] Payouts instance: {:#?}", payouts);

    let mut transactions = Vec::<PayoutRequestTransaction>::new();

    let tsx = PayoutRequestTransaction {
        name: None,
        account: "254717135176".to_string(),
        id_number: None,
        bank_code: None,
        amount: Decimal::new(2000, 2),
        narrative: None,
        category_name: None,
        account_type: None,
        account_reference: None,
    };
    transactions.push(tsx);

    let payout_request = PayoutRequest {
        currency: Currency::Kes,
        provider: Some(PayoutProvider::MpesaB2c),
        device_id: None,
        callback_url: None,
        batch_reference: None,
        transactions,
        // requires_approval: PayoutApproval::Yes,
    };

    let mpesa_payout = payouts.mpesa_b2c(payout_request.clone()).await?;
    println!("Mpesa payout: {:#?}", mpesa_payout);
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let mpesa_approval_request = PayoutApprovalRequest {
        tracking_id: mpesa_payout.tracking_id.clone().unwrap(),
        batch_reference: mpesa_payout.batch_reference.unwrap_or_default(),
        nonce: mpesa_payout.nonce.unwrap(),
        wallet: Some(mpesa_payout.wallet.unwrap()),
        transactions: Some(mpesa_payout.transactions.unwrap()),
    };

    let approved_mpesa_payout = payouts.approve(mpesa_approval_request).await?;
    println!("Approved mpesa payout: {:#?}", approved_mpesa_payout);
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let payout_status_request = PayoutStatusRequest {
        tracking_id: mpesa_payout.tracking_id.unwrap(),
    };

    let status_mpesa_payout = payouts.status(payout_status_request).await?;
    println!("Status mpesa payout: {:#?}", status_mpesa_payout);

    let bank_codes = payouts.bank_codes_ke().await?;
    println!("Payouts Bank Codes: {:#?}", bank_codes);

    println!("Lib works!");

    Ok(())
}
