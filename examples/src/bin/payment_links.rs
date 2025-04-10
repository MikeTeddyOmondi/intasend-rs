#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    Currency, Intasend, PaymentLinksAPI, PaymentLinksCreateDetails, Tarrif 
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

    /// PaymentLinksAPI
    let payment_links = intasend.payment_links();
    println!("[#] PaymentLinksAPI instance: {:#?}", payment_links);

    // let payment_links_list = payment_links.list().await?;
    // println!("[#] Payment Links List: {:#?}", &payment_links_list);

    // let payment_link_id = Uuid::parse_str("5284042b-f13d-49c2-abdf-793fbaf9b8e2").unwrap(); // PaymentLink ID: 5284042b-f13d-49c2-abdf-793fbaf9b8e2
    // let payment_link_details = payment_links.details(payment_link_id.to_string()).await?;
    // println!("[#] Payment Link Details: {:#?}", payment_link_details);

    // // Create a Wallet
    // let wallet_label_identifier = "wallet-label".to_string();
    // let label = format!("{}-{}", wallet_label_identifier, Uuid::new_v4());

    let payload = PaymentLinksCreateDetails {
        currency:Currency::Kes, 
        title: format!("{}-{}", String::from("PaymentLink"), Uuid::new_v4()), 
        amount: Some(100), 
        usage_limit: Some(3), 
        is_active: Some(true), 
        mobile_tarrif: Some(Tarrif::BusinessPays), 
        card_tarrif: Some(Tarrif::BusinessPays), 
        redirect_url: None,
        
    };

    let created_payment_link = payment_links.create(payload).await?;
    println!("[#] Payment Link Created: {:#?}", created_payment_link);

    // // Get transactions from a Wallet
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
    // println!(
    //     "[#] Wallet Transfer Response: {:#?}",
    //     wallets_transfer_response
    // );

    // let fund_mpesa_payload = FundMpesaRequest {
    //     method: Provider::Mpesa,
    //     currency: Currency::Kes,
    //     amount: Decimal::new(1000, 2),
    //     wallet_id: "Y7ELXJQ".to_string(),
    //     phone_number: "254717135176".to_string(),
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
    // println!(
    //     "[#] Fund Checkout request (JSON): {:#?}",
    //     serde_json::to_value(&fund_checkout_req)
    // );

    // let fund_checkout_response: FundCheckoutResponse =
    //     wallets_api.fund_checkout(fund_checkout_req).await?;
    // println!("[#] Fund Checkout response: {:#?}", fund_checkout_response);

    // println!("Lib works!");

    Ok(())
}
