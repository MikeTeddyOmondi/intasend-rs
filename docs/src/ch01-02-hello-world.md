# Hello, World!

In this section we are going to setup our project together with the intasend client with the provided API keys.
In our `src/main.rs` file write out this sample code.

```rust
use std::env;
use dotenvy::dotenv;
use rust_decimal::Decimal;

// import `Intasend` Crate
use intasend::Intasend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file
    dotenv().ok();

    let intasend_public_key =
        env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
    let intasend_secret_key =
        env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");

    /// `Intasend` Client
    let intasend = Intasend::new(intasend_public_key, intasend_secret_key, true);
    println!("[#] Hello Intasend instance: {:#?}", intasend);

}   

```

Now run the project with 

```shell
cargo run
``` 

The program should print out `Hello Intasend instance` with the intasend client `Struct`. That will be assurance that you have setup everything well. 