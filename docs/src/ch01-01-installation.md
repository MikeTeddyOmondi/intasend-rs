# Installation

We can now install the `intasend` crate on the project we created earlier.
You can use the `cargo add` command to add it on out Cargo.toml dependencies.

```shell
cargo add intasend
```

We also need other utility crates for our convinience since some are required by the `intasend` crate to fully utilise the SDK.

1. `rust_decimal` crate for formatting the currency amount in decimals.
2. `tokio` crate to handle asynchronous functions. Enable `full` features.
3. `dotenvy` crate to manage our environment variables
4. `serde` & `serde-json` crate for allowing serialisation and deserialisations, and specifically of `JSON` (Javascript Object Notation) data. Enable `derive` features for the `serde` crate.
5. `anyhow` crate for providing a `Result` trait to utilise on our main and other functions
6. `uuid` crate for generating unique globally unique identifiers. Enable `v4` and `serde` features.

Add the following to the dependencies inside the `Cargo.toml` file

```toml
[dependencies]
# ...other previous dependencies
dotenvy = "0.15.7"
serde = { version = "1.0.195", features = ["derive"] }
serde_json = "1.0.111"
tokio = { version = "1.35.1", features = ["full"] }
anyhow = "1.0.79"
rust_decimal = "1.34.2"
uuid = { version = "1.10.0", features = ["v4", "serde"] }

```
