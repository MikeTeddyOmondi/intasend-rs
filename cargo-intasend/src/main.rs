#![allow(unused)]
#![allow(unused_imports)]

// Intasend Crate
use intasend::{
    ChargeRequest, ChargeResponse, Collection, Intasend, MpesaStkPushRequest, PayoutRequest,
    RefundRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let intasend = Intasend::new(
        "ISPubKey_test_91ffc81a-8ac4-419e-8008-7091caa8d73f".to_string(),
        "ISSecretKey_test_15515fe9-fb5d-4362-970e-625532df8181".to_string(),
        true,
    );
    println!("Intasend instance: {:#?}" ,intasend);
    
    let collection: Collection = intasend.collection();
    println!("Collection instance: {:#?}" ,collection);

    // let payload: ChargeRequest = ChargeRequest {
    //     amount: 10,
    //     currency: "KES".to_string(),
    //     recipient: "Recipient".to_string(),
    // };

    // let charge_response: ChargeResponse = collection.charge(payload).await?;
    // println!("{:?}", charge_response);

    let stkpush_request = MpesaStkPushRequest {
        amount: 10,
        currency: "KES".to_string(),
        recipient: "Jon".to_string(),
        method: "method".to_string(),
    };

    let stkpush_response = collection.mpesa_stk_push(stkpush_request).await?;
    println!("Mpesa STK push: {:?}", stkpush_response);

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
