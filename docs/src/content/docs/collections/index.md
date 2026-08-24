---
title: "Collections API"
---

Within the library, the `CollectionsAPI` implements methods for facilitating Mpesa Express for merchant initiated online payments including:

1. M-Pesa STK Push - `/api/v1/payment/mpesa-stk-push/`
2. Querying status of the M-pesa STK push transaction(s) - `/api/v1/payment/status/`

## Worked example

The whole of `examples/src/bin/collections.rs`, threading each call's output into the
next. It is a workspace member, so it is compiled by `cargo build` and `cargo
clippy` — if the SDK changes underneath it, CI fails rather than the docs
quietly going stale.

Run it with credentials in the environment:

```sh
INTASEND_PUBLIC_KEY=... INTASEND_SECRET_KEY=... cargo run --bin collections
```

```rust title="examples/src/bin/collections.rs"
#![allow(unused)]

use dotenvy::dotenv;
use rust_decimal::Decimal;
use std::env;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CollectionsAPI, Intasend, MpesaStkPushRequest, MpesaStkPushResponse, StkPushStatusRequest,
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

    /// CollectionsAPI
    let collections_api: CollectionsAPI = intasend.collection();
    println!("[#] CollectionsAPI instance: {:#?}", collections_api);

    let stkpush_request = MpesaStkPushRequest {
        amount: Decimal::new(1000, 2), // "10.00".to_string(),
        phone_number: "254717135176".to_string(),
        api_ref: None,
        wallet_id: None,
    };

    let stkpush_response: MpesaStkPushResponse =
        collections_api.mpesa_stk_push(stkpush_request).await?;
    println!("[#] Mpesa STK push: {:#?}", stkpush_response);

    println!("[*] Waiting for the collection response...");
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;

    let stkpushstatus_req = StkPushStatusRequest {
        invoice_id: stkpush_response.invoice.unwrap().invoice_id, // "RXX5P8R".to_string(),
        checkout_id: None,
        signature: None,
    };

    let stkpushstatus = collections_api.status(stkpushstatus_req).await?;
    println!("[#] Mpesa STK Push Status Response: {:#?}", stkpushstatus);

    println!("Lib works!");

    Ok(())
}
```

[View on GitHub](https://github.com/MikeTeddyOmondi/intasend-rs/blob/master/examples/src/bin/collections.rs)

