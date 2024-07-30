#![allow(unused)]
#![allow(unused_imports)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CheckoutsAPI, CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse, CollectionsAPI, Currency, Intasend, MpesaStkPushRequest, MpesaStkPushResponse,
    PayoutProvider, PayoutRequest, PayoutTransaction, RefundRequest, StkPushStatusRequest, Wallet,
    WalletCreateDetails, WalletType,
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
    // let checkout: CheckoutsAPI = intasend.checkout();
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

    // println!("[#] Waiting for the checkout details response...");
    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    // // extract field values from checkout_response struct
    // let checkout_details_req = CheckoutDetailsRequest {
    //     checkout_id: checkout_response.id, // "349f8822-f2d1-4b1b-b398-ba99704dcd7d".to_string(),
    //     signature: checkout_response.signature // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyIzNDlmODgyMi1mMmQxLTRiMWItYjM5OC1iYTk5NzA0ZGNkN2QiXSwiaWF0IjoxNzA4ODY5NzQ2LCJleHAiOjE3MDg4NzMzNDYsImFjY291bnRJRCI6IjlRSjlLR1kiLCJyZWZlcmVuY2UiOiIzNDlmODgyMi1mMmQxLTRiMWItYjM5OC1iYTk5NzA0ZGNkN2QifQ.V8N2VypAfDBXj3uzAT2hiGBNVWCg5x2uBIkQm7IjTBA".to_string(),
    // };

    // let checkout_details_response: CheckoutDetailsResponse =
    //     checkout.details(checkout_details_req).await?;
    // println!(
    //     "[#] Checkout details response: {:#?}",
    //     checkout_details_response
    // );

    /// CollectionsAPI
    let collections_api: CollectionsAPI = intasend.collection();
    println!("[#] CollectionsAPI instance: {:#?}", collections_api);

    let stkpush_request = MpesaStkPushRequest {
        amount: Decimal::new(10, 2),
        phone_number: "254717135176".to_string(),
        api_ref: None,
        wallet_id: None,
    };

    let stkpush_response: MpesaStkPushResponse = collections_api.mpesa_stk_push(stkpush_request).await?;
    println!("[#] Mpesa STK push: {:#?}", stkpush_response);

    println!("[#] Waiting for the collection response...");
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    let stkpushstatus_req = StkPushStatusRequest {
        invoice_id: stkpush_response.invoice.unwrap().invoice_id, // "RXX5P8R".to_string(),
        checkout_id: None,
        signature: None,
    };

    let stkpushstatus = collections_api.status(stkpushstatus_req).await?;
    println!("[#] Mpesa STK Push Status Response: {:#?}", stkpushstatus);

    /// RefundsAPI
    // let refunds = intasend.refunds();
    
    // let refund_request = RefundRequest {
    //     amount: 100,
    //     invoice_id: "RXX5P8R".to_string(),
    //     currency: "USD".to_string(),
    //     recipient: "recipient".to_string(),
    //     method: "M-PESA".to_string(),
    // };

    // let refund_list = refunds.list().await?;
    // println!("[#] Refunds List: {:?}", refund_list);

    // let created_refund: Refund = refunds.create(refund_request).await?;
    // println!("[#] Created refund: {:?}", created_refund);

    // let get_refund = refunds.get("chargeback_id".to_string()).await?;
    // println!("[#] Refund: {:?}", get_refund);

    /// PayoutsAPI
    // let payouts = intasend.payouts();
    // println!("[#] Payouts instance: {:#?}", payouts);

    // let mut transactions: Vec<PayoutTransaction> = Vec::new();

    // let tsx = PayoutTransaction {
    //     status: None,
    //     status_code: None,
    //     request_reference_id: None,
    //     name: None,
    //     account: "254717135176".to_string(),
    //     id_number: None,
    //     bank_code: None,
    //     amount: "10".to_string(),
    //     narrative: None,
    // };
    // transactions.push(tsx);

    // let payout_request = PayoutRequest {
    //     currency: "KES".to_string(),
    //     provider: None,
    //     device_id: None,
    //     callback_url: None,
    //     batch_reference: None,
    //     transactions,
    // };

    // let mpesa_payout = payouts.mpesa_b2c(payout_request.clone()).await?;
    // println!("Mpesa payout: {:#?}", mpesa_payout);
    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    // let approve_mpesa_payout = payouts.approve(payout_request.clone()).await?;
    // println!("Approved mpesa payout: {:#?}", approve_mpesa_payout);
    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    // let status_mpesa_payout = payouts.status(payout_request).await?;
    // println!("Status mpesa payout: {:#?}", status_mpesa_payout);

    /// WalletsAPI
    let wallets_api = intasend.wallets();
    println!("[#] Wallets instance: {:#?}", wallets_api);

    // let wallet_list_info = wallets_api.list().await?;
    // println!("[#] Wallet List Info: {:#?}", wallet_list_info);

    // for wallet in wallet_list_info.results {
    //     println!("Wallet ID: {}", wallet.wallet_id);
    //     println!("Wallet Can Disburse: {}", wallet.can_disburse);
    //     println!("Wallet Currency: {}", wallet.currency);
    //     println!("Wallet Balance: {}", wallet.available_balance);
    // }

    // Wallet ID: VRELLZQ
    let wallet_id = String::from("VRELLZQ");
    let wallet_details = wallets_api.details(wallet_id).await?;
    println!("[#] Wallet Details: {:#?}", wallet_details);

    // Create a Wallet
    let wallet_label_identifier = "wallet-label".to_string();
    let label = format!("{}-{}", wallet_label_identifier, Uuid::new_v4());

    let payload = WalletCreateDetails {
        currency: Currency::KES,
        wallet_type: WalletType::Working,
        can_disburse: true,
        label,
    };

    let created_wallet: Wallet = wallets_api.create(payload).await?;
    println!("[#] Wallet Created Details: {:#?}", created_wallet);
    let wallet_list_info = wallets_api.list().await?;
    println!("[#] Wallet List Info: {:#?}", wallet_list_info);

    println!("Lib works!");

    Ok(())
}
