#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    Currency, Intasend, PaymentLinksAPI, PaymentLinksCreateDetails, PaymentLinksUpdateDetails, Tarrif 
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    let intasend_public_key =
        env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    let intasend_secret_key =
        env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");

    // Intasend Client
    let intasend = Intasend::new(intasend_public_key, intasend_secret_key, true);
    // println!("[#] Intasend instance: {:#?}", intasend);

    // PaymentLinksAPI
    let payment_links = intasend.payment_links();
    println!("[#] PaymentLinksAPI instance: {:#?}", payment_links);

    // List all payment links
    let payment_links_list = payment_links.list().await?;
    println!("[#] Payment Links List: {:#?}", &payment_links_list);

    // Query payment link details
    let payment_link_id = Uuid::parse_str("0bd8984a-f487-46fb-b7b6-c17f8e87ccc8").unwrap(); // PaymentLink ID: 5284042b-f13d-49c2-abdf-793fbaf9b8e2
    let payment_link_details = payment_links.details(payment_link_id.to_string()).await?;
    println!("[#] Payment Link Details: {:#?}", payment_link_details);

    // Create payment link(s)
    let uid = Uuid::new_v4();
    let payment_link_label_identifier = "payment-link-label".to_string();
    let payment_link_label = format!("{}-{}", payment_link_label_identifier, uid);

    let payload = PaymentLinksCreateDetails {
        currency:Currency::Kes, 
        title: payment_link_label, 
        amount: Some(100), 
        usage_limit: Some(3), 
        is_active: Some(true), 
        mobile_tarrif: Some(Tarrif::BusinessPays), 
        card_tarrif: Some(Tarrif::BusinessPays), 
        redirect_url: None,
        
    };

    let created_payment_link = payment_links.create(payload).await?;
    println!("[#] Payment Link Created: {:#?}", created_payment_link);

    // Update payment link(s)
    let uid = Uuid::parse_str("e4f6126d-b374-4edb-bf17-f9240d24d66e").unwrap();
    let payment_link_label_identifier = "payment-link-updated".to_string();
    let payment_link_label_updated = format!("{}-{}", payment_link_label_identifier, uid);

    let payload = PaymentLinksUpdateDetails {
        currency:Currency::Kes, 
        title: payment_link_label_updated, 
        amount: Some(100), 
        usage_limit: Some(3), 
        is_active: Some(true), 
        mobile_tarrif: Some(Tarrif::BusinessPays), 
        card_tarrif: Some(Tarrif::BusinessPays), 
        redirect_url: None,
        
    };

    let updated_payment_link = payment_links.update(uid.to_string(), payload).await?;
    println!("[#] Payment Link Updated: {:#?}", updated_payment_link);

    println!("Lib works!");

    Ok(())
}
