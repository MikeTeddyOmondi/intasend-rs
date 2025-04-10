# M-pesa Stk Push

Initiate a M-pesa STK push from a user's phone given the phone number

Import the modules required:

```rust
use rust_decimal::Decimal;
use uuid::Uuid;

// Intasend Crate
use intasend::{
    CollectionsAPI,
    MpesaStkPushRequest, 
    MpesaStkPushResponse,
};

```

Then call the `mpesa_stk_push()` function:
```rust

/// CollectionsAPI
let collections_api: CollectionsAPI = intasend.collection();

let stkpush_request = MpesaStkPushRequest {
    amount: Decimal::new(1000, 2), // "10.00".to_string(),
    phone_number: "254111234567".to_string(),
    api_ref: None,
    wallet_id: None,
};

let stkpush_response: MpesaStkPushResponse =
    collections_api.mpesa_stk_push(stkpush_request).await?;
println!("[#] Mpesa STK push: {:#?}", stkpush_response);

```