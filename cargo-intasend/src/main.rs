#![allow(unused)]
#![allow(unused_imports)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;

// Intasend Crate
use intasend::{
    Checkout, CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse, Collection, Currency, Intasend, MpesaStkPushRequest, MpesaStkPushResponse,
    PayoutRequest, RefundRequest, StkPushStatusRequest,
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

    /// Checkout API
    // let checkout: Checkout = intasend.checkout();
    // // println!("[#] Checkout instance: {:#?}", checkout);

    // let checkout_req = CheckoutRequest {
    //     first_name: Some("Joe".to_string()),
    //     last_name: Some("Doe".to_string()),
    //     email: Some("joe@doe.com".to_string()),
    //     method: Some(CheckoutMethod::MPESA.as_str()),
    //     amount: Decimal::new(100, 2),
    //     currency: Currency::KES,
    // };
    // println!("[#] Checkout request: {:#?}", checkout_req);

    // let checkout_response: CheckoutResponse = checkout.initiate(checkout_req).await?;
    // println!("[#] Checkout response: {:#?}", checkout_response);

    // println!("[#] Waiting for the checkout response...");
    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    // // extract field values from checkout_response struct
    // let checkout_details_req = CheckoutDetailsRequest {
    //     checkout_id: checkout_response.id, // "349f8822-f2d1-4b1b-b398-ba99704dcd7d".to_string(),
    //     signature: checkout_response.signature //"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyIzNDlmODgyMi1mMmQxLTRiMWItYjM5OC1iYTk5NzA0ZGNkN2QiXSwiaWF0IjoxNzA4ODY5NzQ2LCJleHAiOjE3MDg4NzMzNDYsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiIzNDlmODgyMi1mMmQxLTRiMWItYjM5OC1iYTk5NzA0ZGNkN2QifQ.V8N2VypAfDBXj3uzAT2hiGBNVWCg5x2uBIkQm7IjTBA".to_string(),
    // };

    // let checkout_details_response: CheckoutDetailsResponse =
    //     checkout.details(checkout_details_req).await?;
    // println!(
    //     "[#] Checkout details response: {:#?}",
    //     checkout_details_response
    // );

    /// Collection API
    let collection: Collection = intasend.collection();
    println!("[#] Collection instance: {:#?}", collection);

    let stkpush_request = MpesaStkPushRequest {
        amount: 10,
        phone_number: "254717135176".to_string(),
        api_ref: None,
        wallet_id: None,
    };

    let stkpush_response: MpesaStkPushResponse = collection.mpesa_stk_push(stkpush_request).await?;
    println!("[#] Mpesa STK push: {:#?}", stkpush_response);

    println!("[#] Waiting for the collection response...");
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    let stkpushstatus_req = StkPushStatusRequest {
        invoice_id: stkpush_response.invoice.unwrap().invoice_id, // "RXX5P8R".to_string(),
        checkout_id: None,
        signature: None,
    };

    let stkpushstatus = collection.status(stkpushstatus_req).await?;
    println!("[#] Mpesa STK Push Status Response: {:#?}", stkpushstatus);

    /// Refunds API
    // let refund_request = RefundRequest {
    //     amount: 100,
    //     invoice_id: "RXX5P8R".to_string(),
    //     currency: "USD".to_string(),
    //     recipient: "recipient".to_string(),
    //     method: "M-PESA".to_string(),
    // };

    // let refunds = intasend.refunds();

    // let refund_list = refunds.list().await?;
    // println!("[#] Refunds List: {:?}", refund_list);

    // let created_refund: Refund = refunds.create(refund_request).await?;
    // println!("[#] Created refund: {:?}", created_refund);

    // let get_refund = refunds.get("chargeback_id".to_string()).await?;
    // println!("[#] Refund: {:?}", get_refund);

    /// Payouts API
    // let payouts = intasend.payouts();
    // println!("[#] Payouts instance: {:#?}", payouts);

    // let payout_request = PayoutRequest {
    //     currency: "KES".to_string(),
    //     provider: "MPESA-B2C".to_string(),
    //     device_id: None,
    //     callback_url: None,
    //     batch_reference: None,
    // };

    // let mpesa_payout = payouts.mpesa_b2c(payout_request).await?;
    // // let approve_mpesa_payout = payouts.approve(payout_request).await?;
    // // let status_mpesa_payout = payouts.status(payout_request).await?;
    // println!("Mpesa payout: {:?}", mpesa_payout);

    /// Wallets API
    // let wallets_api = intasend.wallets();
    // println!("[#] Wallets instance: {:#?}", wallets);

    // let wallets = wallets_api.list().await?;

    // for wallet in wallets {
    //     println!("Wallet ID: {}", wallet.id);
    //     println!("Wallet Name: {}", wallet.name);
    //     println!("Wallet Currency: {}", wallet.currency);
    //     println!("Wallet Balance: {}", wallet.balance);
    // }

    println!("Lib works!");

    Ok(())
}
