#![allow(unused)]
#![allow(unused_imports)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest, CheckoutResponse, CheckoutsAPI, CollectionsAPI, Currency, Intasend, MpesaStkPushRequest, MpesaStkPushResponse, PayoutProvider, PayoutRequest, PayoutTransaction, Refund, RefundReason, RefundRequest, StkPushStatusRequest, Wallet, WalletCreateDetails, WalletType
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
    //     method: Some(CheckoutMethod::MPESA),
    //     amount: Decimal::new(10000, 2), // 100.00
    //     currency: Currency::Kes,
    // };
    // println!("[#] Checkout request: {:#?}", checkout_req);
    // println!("[#] Checkout request (JSON): {:#?}", serde_json::to_value(&checkout_req));

    // let checkout_response: CheckoutResponse = checkout.initiate(checkout_req).await?;
    // println!("[#] Checkout response: {:#?}", checkout_response);

    // println!("[*] Waiting for the checkout details response...");
    // tokio::time::sleep(std::time::Duration::from_secs(45)).await;

    // // extract field values from checkout_response struct
    // let checkout_details_req = CheckoutDetailsRequest {
    //     checkout_id: checkout_response.id, // "c33e0d25-d290-4cda-8d7e-02ff38544a46".to_string(),
    //     signature:  checkout_response.signature, // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyJjMzNlMGQyNS1kMjkwLTRjZGEtOGQ3ZS0wMmZmMzg1NDRhNDYiXSwiaWF0IjoxNzIyMzUzMzU1LCJleHAiOjE3MjIzNTY5NTUsImFjY291bnRJRCI6IlpRTTBNOFkiLCJyZWZlcmVuY2UiOiJjMzNlMGQyNS1kMjkwLTRjZGEtOGQ3ZS0wMmZmMzg1NDRhNDYifQ.dZMFD-2k8OnnvvsF8oLEZvEprU6oiBPfCdv0M7tKdnI".to_string(),
    // };

    // let checkout_details_response: CheckoutDetailsResponse =
    //     checkout.details(checkout_details_req).await?;
    // println!(
    //     "[#] Checkout details response: {:#?}",
    //     checkout_details_response
    // );

    /// CollectionsAPI
    // let collections_api: CollectionsAPI = intasend.collection();
    // println!("[#] CollectionsAPI instance: {:#?}", collections_api);

    // let stkpush_request = MpesaStkPushRequest {
    //     amount: Decimal::new(1000, 2), // "10.00".to_string(),
    //     phone_number: "254717135176".to_string(),
    //     api_ref: None,
    //     wallet_id: None,
    // };

    // let stkpush_response: MpesaStkPushResponse = collections_api.mpesa_stk_push(stkpush_request).await?;
    // println!("[#] Mpesa STK push: {:#?}", stkpush_response);

    // println!("[*] Waiting for the collection response...");
    // tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    // let stkpushstatus_req = StkPushStatusRequest {
    //     invoice_id: stkpush_response.invoice.unwrap().invoice_id, // "RXX5P8R".to_string(),
    //     checkout_id: None,
    //     signature: None,
    // };

    // let stkpushstatus = collections_api.status(stkpushstatus_req).await?;
    // println!("[#] Mpesa STK Push Status Response: {:#?}", stkpushstatus);

    /// RefundsAPI
    // let refunds_api = intasend.refunds();
    // println!("[#] RefundsAPI instance: {:#?}", refunds_api);
    
    // let refund_request = RefundRequest {
    //     amount: Decimal::new(1000, 2),
    //     invoice: "RXX5P8R".to_string(),
    //     reason: RefundReason::UnavailableService, 
    //     reason_details: "The service was unavailable".to_string(), 
    //     // currency: Currency::Kes,
    //     // method: "M-PESA".to_string(),
    //     // recipient: "recipient".to_string(),
    // };

    // let refund_list = refunds_api.list().await?;
    // println!("[#] Refunds List: {:?}", refund_list);

    // let created_refund: Refund = refunds_api.create(refund_request).await?;
    // println!("[#] Created refund: {:?}", created_refund);

    // let get_refund = refunds_api.get("chargeback_id".to_string()).await?;
    // println!("[#] Refund: {:?}", get_refund);

    /// PayoutsAPI
    let payouts = intasend.payouts();
    println!("[#] Payouts instance: {:#?}", payouts);

    let mut transactions = Vec::<PayoutTransaction>::new();

    let tsx = PayoutTransaction {
        status: None,
        status_code: None,
        request_reference_id: None,
        name: None,
        account: "254717135176".to_string(),
        id_number: None,
        bank_code: None,
        amount: "10".to_string(),
        narrative: None,
    };
    transactions.push(tsx);

    let payout_request = PayoutRequest {
        currency: "KES".to_string(),
        provider: None,
        device_id: None,
        callback_url: None,
        batch_reference: None,
        transactions,
    };

    let mpesa_payout = payouts.mpesa_b2c(payout_request.clone()).await?;
    println!("Mpesa payout: {:#?}", mpesa_payout);
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    let approve_mpesa_payout = payouts.approve(payout_request.clone()).await?;
    println!("Approved mpesa payout: {:#?}", approve_mpesa_payout);
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    let status_mpesa_payout = payouts.status(payout_request).await?;
    println!("Status mpesa payout: {:#?}", status_mpesa_payout);

    /// WalletsAPI
    // let wallets_api = intasend.wallets();
    // println!("[#] Wallets instance: {:#?}", wallets_api);

    // let wallet_list_info = wallets_api.list().await?;
    // println!("[#] Wallet List Info: {:#?}", wallet_list_info);

    // for wallet in wallet_list_info.results {
    //     println!("Wallet ID: {}", wallet.wallet_id);
    //     println!("Wallet Can Disburse: {}", wallet.can_disburse);
    //     println!("Wallet Currency: {}", wallet.currency);
    //     println!("Wallet Balance: {}", wallet.available_balance);
    // }

    // Wallet ID: VRELLZQ
    // let wallet_id = String::from("VRELLZQ");
    // let wallet_details = wallets_api.details(wallet_id).await?;
    // println!("[#] Wallet Details: {:#?}", wallet_details);

    // // Create a Wallet
    // let wallet_label_identifier = "wallet-label".to_string();
    // let label = format!("{}-{}", wallet_label_identifier, Uuid::new_v4());

    // let payload = WalletCreateDetails {
    //     currency: Currency::KES,
    //     wallet_type: WalletType::Working,
    //     can_disburse: true,
    //     label,
    // };

    // let created_wallet: Wallet = wallets_api.create(payload).await?;
    // println!("[#] Wallet Created Details: {:#?}", created_wallet);
    // let wallet_list_info = wallets_api.list().await?;
    // println!("[#] Wallet List Info: {:#?}", wallet_list_info);

    println!("Lib works!");

    Ok(())
}
