#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    Currency, FundCheckoutRequest, FundCheckoutResponse, FundMpesaRequest, Intasend, Provider,
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

    /// WalletsAPI
    let wallets_api = intasend.wallets();
    println!("[#] Wallets instance: {:#?}", wallets_api);

    let wallet_list_info = wallets_api.list().await?;
    println!("[#] Wallet List Info: {:#?}", &wallet_list_info);

    for wallet in wallet_list_info.results {
        println!("Wallet ID: {}", wallet.wallet_id);
        println!("Wallet Can Disburse: {}", wallet.can_disburse);
        println!("Wallet Currency: {:?}", wallet.currency);
        println!("Wallet Balance: {}", wallet.current_balance);
    }

    let wallet_id = String::from("Y7ELXJQ"); // Wallet ID: VRELLZQ
    let wallet_details = wallets_api.details(wallet_id.clone()).await?;
    println!("[#] Wallet Details: {:#?}", wallet_details);

    // Create a Wallet
    let wallet_label_identifier = "wallet-label".to_string();
    let label = format!("{}-{}", wallet_label_identifier, Uuid::new_v4());

    let payload = WalletCreateDetails {
        currency: Currency::Kes,
        wallet_type: WalletType::Working,
        can_disburse: true,
        label,
    };

    let created_wallet: Wallet = wallets_api.create(payload).await?;
    println!("[#] Wallet Created Details: {:#?}", created_wallet);
    // let wallet_list_info = wallets_api.list().await?;
    // println!("[#] Wallet List Info: {:#?}", wallet_list_info);

    // Get transactions from a Wallet
    let wallet_transanctions = wallets_api.transactions(wallet_id.clone()).await?;
    println!("[#] Wallet Transactions: {:#?}", wallet_transanctions);

    let source_wallet_id = wallet_id.clone(); // "VRELLZQ".to_string();
    let intra_transfer_payload = WalletIntraTransferRequest {
        wallet_id: "VRELLZQ".to_string(), // "Y7ELXJQ".to_string();
        amount: Decimal::new(10000, 2),
        narrative: "wallet funding".to_string(),
    };
    let wallets_transfer_response = wallets_api
        .intra_transfer(source_wallet_id, intra_transfer_payload)
        .await?;
    println!(
        "[#] Wallet Transfer Response: {:#?}",
        wallets_transfer_response
    );

    let fund_mpesa_payload = FundMpesaRequest {
        method: Provider::Mpesa,
        currency: Currency::Kes,
        amount: Decimal::new(1000, 2),
        wallet_id: "Y7ELXJQ".to_string(),
        phone_number: "254717135176".to_string(),
    };

    let fund_mpesa_response = wallets_api.fund_mpesa(fund_mpesa_payload).await?;
    println!("[#] Wallet Fund Mpesa Response: {:#?}", fund_mpesa_response);
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    let wallet_details = wallets_api.details("Y7ELXJQ".to_string()).await?;
    println!("[#] Wallet Details: {:#?}", wallet_details);

    let fund_checkout_req = FundCheckoutRequest {
        first_name: Some("Foo".to_string()),
        last_name: Some("Bar".to_string()),
        email: Some("foobar@baz.com".to_string()),
        method: Some(Provider::Bank),
        amount: Decimal::new(100000, 2), // 1000.00
        currency: Currency::Kes,
        wallet_id: "Y7ELXJQ".to_string(),
        api_ref: None,
        redirect_url: None,
    };
    println!("[#] Fund Checkout Request: {:#?}", fund_checkout_req);
    println!(
        "[#] Fund Checkout request (JSON): {:#?}",
        serde_json::to_value(&fund_checkout_req)
    );

    let fund_checkout_response: FundCheckoutResponse =
        wallets_api.fund_checkout(fund_checkout_req).await?;
    println!("[#] Fund Checkout response: {:#?}", fund_checkout_response);

    println!("Lib works!");

    Ok(())
}
