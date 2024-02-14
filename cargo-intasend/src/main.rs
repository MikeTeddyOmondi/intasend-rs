#![allow(unused)]
#![allow(unused_imports)]

use rust_decimal::Decimal;
// Intasend Crate
use intasend::{
    CheckoutMethod, CheckoutRequest, CheckoutResponse, Collection, Currency, Intasend, MpesaStkPushRequest, MpesaStkPushResponse, PayoutRequest, RefundRequest, StatusRequest
};

// ISPubKey_test_c1f90113-3dbb-4201-9b88-f1c2d3056e5c
// ISSecretKey_test_5527b085-40b6-460a-9c31-25d58a204d20

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// Intasend Client
    let intasend = Intasend::new(
        "ISPubKey_test_c1f90113-3dbb-4201-9b88-f1c2d3056e5c".to_string(),
        "ISSecretKey_test_5527b085-40b6-460a-9c31-25d58a204d20".to_string(),
        true,
    );
    println!("Intasend instance: {:#?}", intasend);

    /// Checkout API
    let checkout = intasend.checkout();
    println!("Checkout instance: {:#?}", checkout);

    let checkout_req = CheckoutRequest {
        first_name: Some("Joe".to_string()),
        last_name: Some("Doe".to_string()),
        email: Some("joe@doe.com".to_string()),
        method: Some(CheckoutMethod::CARDPAYMENT),
        amount: Decimal::new(10, 2),
        currency: Currency::USD,
    };

    let checkout_response: CheckoutResponse = checkout.initiate(checkout_req).await?;
    println!("{:?}", checkout_response);

    /// Collection API
    // let collection: Collection = intasend.collection();
    // println!("Collection instance: {:#?}", collection);

    // let stkpush_request = MpesaStkPushRequest {
    //     amount: 10,
    //     phone_number: "254717135176".to_string(),
    //     api_ref: None,
    //     wallet_id: None,
    // };

    // let stkpush_response: MpesaStkPushResponse = collection.mpesa_stk_push(stkpush_request).await?;
    // println!("[#] Mpesa STK push: {:#?}", stkpush_response);

    // tokio::time::sleep(std::time::Duration::from_secs(10));

    // let stkpushstatus_req: StatusRequest = StatusRequest {
    //     checkout_id: todo!(),
    //     invoice_id: todo!(),
    //     signature: todo!(),
    // };

    // let stkpushstatus = collection.status(stkpushstatus_req).await?;
    // println!(
    //     "[#] Mpesa STK Push Status Response: {:#?}",
    //     stkpushstatus
    // );

    // let refund_request = RefundRequest {
    //     amount: 100,
    //     currency: "USD".to_string(),
    //     recipient: "recipient".to_string(),
    //     method: "M-PESA".to_string(),
    // };

    // let refunds = intasend.refunds();

    // let create_refund = refunds.create(refund_request).await?;
    // println!("Created refund: {:?}", create_refund);

    // let get_refund = refunds.get("chargeback_id".to_string()).await?;
    // println!("Get refund: {:?}", get_refund);

    // let payouts = intasend.payouts();

    // let payout_request = PayoutRequest {
    //     amount: 100,
    //     currency: "USD".to_string(),
    //     recipient: "recipient".to_string(),
    //     method: "MPESA-B2C".to_string(),
    // };

    // let mpesa_payout = payouts.mpesa(payout_request).await?;
    // // let approve_mpesa_payout = payouts.approve(payout_request).await?;
    // // let status_mpesa_payout = payouts.status(payout_request).await?;
    // println!("Mpesa payout: {:?}", mpesa_payout);

    // let wallets = intasend.wallets();

    // let wallets = wallets.list().await?;

    // for wallet in wallets {
    //     println!("Wallet ID: {}", wallet.id);
    //     println!("Wallet Name: {}", wallet.name);
    //     println!("Wallet Currency: {}", wallet.currency);
    //     println!("Wallet Balance: {}", wallet.balance);
    // }

    println!("Lib works!");

    Ok(())
}
