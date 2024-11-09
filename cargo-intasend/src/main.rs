#![allow(unused)]
#![allow(unused_imports)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse, CheckoutsAPI, CollectionsAPI, Currency, FundCheckoutRequest,
    FundCheckoutResponse, FundMpesaRequest, Intasend, MpesaStkPushRequest, MpesaStkPushResponse,
    PayoutApproval, PayoutApprovalRequest, PayoutProvider, PayoutRequest, PayoutRequestTransaction,
    PayoutStatusRequest, Provider, Refund, RefundReason, RefundRequest, StkPushStatusRequest,
    Wallet, WalletCreateDetails, WalletIntraTransferRequest, WalletType,
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
    println!("[#] Checkout request (JSON): {:#?}", serde_json::to_value(&checkout_req));

    let checkout_response: CheckoutResponse = checkout.initiate(checkout_req).await?;
    println!("[#] Checkout response: {:#?}", checkout_response);

    println!("[*] Waiting for the checkout details response...");
    tokio::time::sleep(std::time::Duration::from_secs(45)).await;

    // extract field values from checkout_response struct
    let checkout_details_req = CheckoutDetailsRequest {
        checkout_id: checkout_response.id, // "c33e0d25-d290-4cda-8d7e-02ff38544a46".to_string(),
        signature:  checkout_response.signature, // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZSI6ImV4cHJlc3MtY2hlY2tvdXQiLCJpc3MiOiJJbnRhU2VuZCBTb2x1dGlvbnMgTGltaXRlZCIsImF1ZCI6WyJjMzNlMGQyNS1kMjkwLTRjZGEtOGQ3ZS0wMmZmMzg1NDRhNDYiXSwiaWF0IjoxNzIyMzUzMzU1LCJleHAiOjE3MjIzNTY5NTUsImFjY291bnRJRCI6IlpRTTBNOFkiLCJyZWZlcmVuY2UiOiJjMzNlMGQyNS1kMjkwLTRjZGEtOGQ3ZS0wMmZmMzg1NDRhNDYifQ.dZMFD-2k8OnnvvsF8oLEZvEprU6oiBPfCdv0M7tKdnI".to_string(),
    };

    let checkout_details_response: CheckoutDetailsResponse =
        checkout.details(checkout_details_req).await?;
    println!(
        "[#] Checkout details response: {:#?}",
        checkout_details_response
    );

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
    //     invoice: "YVB845R".to_string(),
    //     reason: RefundReason::UnavailableService,
    //     reason_details: "The service was unavailable".to_string(),
    // };

    // let refund_list = refunds_api.list().await?;
    // println!("[#] Refunds List: {:#?}", refund_list);

    // let created_refund: Refund = refunds_api.create(refund_request).await?;
    // println!("[#] Created refund: {:?}", created_refund);

    // let get_refund = refunds_api.get("JRO97YK".to_string()).await?; // JRO97YK, ERG2PRK
    // println!("[#] Refund: {:#?}", get_refund);

    /// PayoutsAPI
    // let payouts = intasend.payouts();
    // println!("[#] Payouts instance: {:#?}", payouts);

    // let mut transactions = Vec::<PayoutRequestTransaction>::new();

    // let tsx = PayoutRequestTransaction {
    //     name: None,
    //     account: "254717135176".to_string(),
    //     id_number: None,
    //     bank_code: None,
    //     amount: Decimal::new(2000, 2),
    //     narrative: None,
    //     category_name: None,
    //     account_type: None,
    //     account_reference: None
    // };
    // transactions.push(tsx);

    // let payout_request = PayoutRequest {
    //     currency: Currency::Kes,
    //     provider: Some(PayoutProvider::MpesaB2c),
    //     device_id: None,
    //     callback_url: None,
    //     batch_reference: None,
    //     transactions,
    //     // requires_approval: PayoutApproval::Yes,
    // };

    // let mpesa_payout = payouts.mpesa_b2c(payout_request.clone()).await?;
    // println!("Mpesa payout: {:#?}", mpesa_payout);
    // tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // let mpesa_approval_request = PayoutApprovalRequest {
    //   tracking_id: mpesa_payout.tracking_id.clone().unwrap(),
    //   batch_reference: mpesa_payout.batch_reference.unwrap_or_default(),
    //   nonce: mpesa_payout.nonce.unwrap(),
    //   wallet: Some(mpesa_payout.wallet.unwrap()),
    //   transactions: Some(mpesa_payout.transactions.unwrap())
    // };

    // let approved_mpesa_payout = payouts.approve(mpesa_approval_request).await?;
    // println!("Approved mpesa payout: {:#?}", approved_mpesa_payout);
    // tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // let payout_status_request = PayoutStatusRequest {
    //   tracking_id: mpesa_payout.tracking_id.unwrap()
    // };

    // let status_mpesa_payout = payouts.status(payout_status_request).await?;
    // println!("Status mpesa payout: {:#?}", status_mpesa_payout);

    // let bank_codes = payouts.bank_codes_ke().await?;
    // println!("Payouts Bank Codes: {:#?}", bank_codes);

    /// WalletsAPI
    // let wallets_api = intasend.wallets();
    // println!("[#] Wallets instance: {:#?}", wallets_api);

    // let wallet_list_info = wallets_api.list().await?;
    // println!("[#] Wallet List Info: {:#?}", &wallet_list_info);

    // for wallet in wallet_list_info.results {
    //     println!("Wallet ID: {}", wallet.wallet_id);
    //     println!("Wallet Can Disburse: {}", wallet.can_disburse);
    //     println!("Wallet Currency: {:?}", wallet.currency);
    //     println!("Wallet Balance: {}", wallet.current_balance);
    // }

    // let wallet_id = String::from("Y7ELXJQ"); // Wallet ID: VRELLZQ
    // let wallet_details = wallets_api.details(wallet_id.clone()).await?;
    // println!("[#] Wallet Details: {:#?}", wallet_details);

    // // Create a Wallet
    // let wallet_label_identifier = "wallet-label".to_string();
    // let label = format!("{}-{}", wallet_label_identifier, Uuid::new_v4());

    // let payload = WalletCreateDetails {
    //     currency: Currency::Kes,
    //     wallet_type: WalletType::Working,
    //     can_disburse: true,
    //     label,
    // };

    // let created_wallet: Wallet = wallets_api.create(payload).await?;
    // println!("[#] Wallet Created Details: {:#?}", created_wallet);
    // // let wallet_list_info = wallets_api.list().await?;
    // // println!("[#] Wallet List Info: {:#?}", wallet_list_info);

    // Get transactions from a Wallet
    // let wallet_transanctions = wallets_api.transactions(wallet_id.clone()).await?;
    // println!("[#] Wallet Transactions: {:#?}", wallet_transanctions);

    // let source_wallet_id = wallet_id.clone(); // "VRELLZQ".to_string();
    // let intra_transfer_payload = WalletIntraTransferRequest {
    //     wallet_id: "VRELLZQ".to_string(), // "Y7ELXJQ".to_string();
    //     amount: Decimal::new(10000, 2),
    //     narrative: "wallet funding".to_string(),
    // };
    // let wallets_transfer_response = wallets_api
    //     .intra_transfer(source_wallet_id, intra_transfer_payload)
    //     .await?;
    // println!("[#] Wallet Transfer Response: {:#?}", wallets_transfer_response);

    // let fund_mpesa_payload = FundMpesaRequest {
    //   method: Provider::Mpesa,
    //   currency: Currency::Kes,
    //   amount: Decimal::new(1000, 2),
    //   wallet_id: "Y7ELXJQ".to_string(),
    //   phone_number: "254717135176".to_string(),
    // };

    // let fund_mpesa_response = wallets_api.fund_mpesa(fund_mpesa_payload).await?;
    // println!("[#] Wallet Fund Mpesa Response: {:#?}", fund_mpesa_response);
    // tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    // let wallet_details = wallets_api.details("Y7ELXJQ".to_string()).await?;
    // println!("[#] Wallet Details: {:#?}", wallet_details);

    // let fund_checkout_req = FundCheckoutRequest {
    //     first_name: Some("Foo".to_string()),
    //     last_name: Some("Bar".to_string()),
    //     email: Some("foobar@baz.com".to_string()),
    //     method: Some(Provider::Bank),
    //     amount: Decimal::new(100000, 2), // 1000.00
    //     currency: Currency::Kes,
    //     wallet_id: "Y7ELXJQ".to_string(),
    //     api_ref: None,
    //     redirect_url: None,
    // };
    // println!("[#] Fund Checkout Request: {:#?}", fund_checkout_req);
    // println!("[#] Fund Checkout request (JSON): {:#?}", serde_json::to_value(&fund_checkout_req));

    // let fund_checkout_response: FundCheckoutResponse = wallets_api.fund_checkout(fund_checkout_req).await?;
    // println!("[#] Fund Checkout response: {:#?}", fund_checkout_response);

    println!("Lib works!");

    Ok(())
}
