# M-pesa Stk Push Status

Query the status of an M-pesa STK push transaction initiated on a user's phone

Import the modules required:

```rust
use rust_decimal::Decimal;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CollectionsAPI, 
    Intasend, 
    MpesaStkPushRequest, 
    MpesaStkPushResponse, 
    StkPushStatusRequest
};

```

After calling the STK push `mpesa_stk_push()` function you can then call the `status()` function 
passing in the `invoice_id` from the STK push response inside the `invoice` field.

```rust

/// CollectionsAPI
let collections_api: CollectionsAPI = intasend.collection();

let stkpush_request = MpesaStkPushRequest {
    amount: Decimal::new(1000, 2), // "10.00".to_string(),
    phone_number: "254717135176".to_string(),
    api_ref: None,
    wallet_id: None,
};

let stkpush_response: MpesaStkPushResponse =
    collections_api.mpesa_stk_push(stkpush_request).await?;

// Wait for 30 seconds and query for the STK push status
tokio::time::sleep(std::time::Duration::from_secs(30)).await;

let stkpushstatus_req = StkPushStatusRequest {
    invoice_id: stkpush_response.invoice.unwrap().invoice_id,
    checkout_id: None,
    signature: None,
};

let stkpushstatus = collections_api.status(stkpushstatus_req).await?;
println!("[#] Mpesa STK Push Status Response: {:#?}", stkpushstatus);

```