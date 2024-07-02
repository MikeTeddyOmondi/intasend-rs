# [Changelog](https://github.com/MikeTeddyOmondi/intasend-rs/CHANGELOG.md)

## 0.2.0

### Breaking Changes

- Updates:
  - updated `crates.io` docs at doc.rs
  - Added struct identifiers
  - version: `0.2.0` has `Payouts`, `Chargebacks & Refunds` and `Transfers` IntaSend API support, bug fixes, refactored code

## 0.1.6

### Patch Changes

- Updates:
  - updated crate docs
  - added new structs: `Tarrif`, `PaymentLink`
  - changed struct identifiers from `StatusRequest`, `StatusResponse` to `StkPushStatusRequest`, `StkPushStatusResponse`
  - version: `0.1.6` has bug fixes and refactored code

## 0.1.5

### Patch Changes

- Updates: Added `CHANGELOG.md` for tracking changes
- Updates: added crate docs to `checkout` module and refactored code
- Updates: added crate docs to `collection` module and refactored code
- Updates: added crate docs to `intasend` module
- Updates: version: `0.1.5` has additional feature toggles for client/server apps

## 0.1.4

### Patch Changes

- Updates: separated `checkout` module out of the `collection` module

## 0.1.3

### Patch Changes

- Updates: fixed crate `README.md`
- Updates: fixed `send` method of the `intasend` client
- Updates: stabilised `collection.mpesa_stk_push()` function

## 0.1.2

### Patch Changes

- Updates: Added `rust_decimal` crate for floating point values
- Updates: Added `cargo-intasend` bin package for testing the `intasend` crate
- Updates: Added `anyhow` crate

## 0.1.1

### Patch Changes

- Updates: stabilising the Intasend Core
- Updates: collection API: added `collection.mpesa_stk_push()` function

## 0.1.0

### Patch Changes

- Initial release [a764ba1b]
