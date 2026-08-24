# [Changelog](https://github.com/MikeTeddyOmondi/intasend-rs/CHANGELOG.md)

## Unreleased

### Changed

- Documentation moved from mdBook to Astro Starlight, deployed to Cloudflare at
  `intasend-rs.pages.dev`. All 33 pages carried across mechanically — titles
  taken from each page's `h1`, relative links rewritten — with no rewording.

  Each API overview embeds the whole of its file from `examples/src/bin/`. Those
  are workspace members, so `cargo build` and `cargo clippy` compile them: when
  the SDK changes underneath, CI fails rather than the documentation going
  quietly stale.

  Adds the Subscriptions page the docs never had, noting that its shapes were
  verified against the OpenAPI definition but never called against the sandbox,
  and that its currency set differs from the collection endpoints.

  Known gap: 28 of the 33 migrated pages are prose with no code. The overviews
  now carry worked examples; the per-method pages beneath them do not.

- CI split in two. `ci.yml` validates every pull request and publishes nothing;
  `release.yml` runs only on a `v*.*.*` tag and refuses unless the tag matches
  `Cargo.toml` and the changelog mentions it. Previously a single workflow ran
  `cargo publish` on every push to master *and* every pull request, gated only
  by the version already existing on crates.io.

### Fixed

- `Currency` gains `GHS`, `NGN`, `UGX`, `TZS`, `XAF` and `XOF`. The
  Subscriptions plan endpoint accepts these and the enum did not, so six
  markets — Ghana, Nigeria, Uganda, Tanzania and both CFA zones — could not be
  expressed at all. `KES` was the only variant that worked there.

  Merged into the shared enum rather than adding a subscriptions-specific one:
  a currency is a currency, and two overlapping enums would push conversion
  onto every caller. Which endpoints accept which variants is documented on the
  variants themselves; `USD`, `EUR` and `GBP` remain valid for collection and
  are rejected by subscriptions.

  Additive, so no caller breaks. `Currency` also gains `PartialEq`/`Eq`.

### Added

- Compile-time assertions that `Intasend`, every `*API` struct and the error
  type are `Send + Sync + 'static`.

  This is what axum needs to hold the client in state and to keep a handler's
  future `Send`. Nothing was wrong — all of them already satisfy it — but the
  guarantee was implicit, and the failure mode is unpleasant: axum reports
  "future cannot be sent between threads safely" against the *handler*, naming
  neither the field nor the type responsible. The assertions fail the build the
  moment a non-`Send` field is added.

  `Intasend` is three owned fields and is `Clone`, so axum state can hold it
  directly. It does not need wrapping in an `Arc`; one only adds a pointer hop
  and the appearance of shared mutability that is not there.

- Tests pinning every `Currency` variant to its ISO wire value, and asserting
  the seven the Subscriptions plan endpoint documents are all representable.

## v0.4.0 - 2026-06-25

### Added

- `SubscriptionsAPI` — full IntaSend Subscriptions API support: plans (`create_plan`, `list_plans`, `plan_details`, `update_plan`), subscriptions (`create`, `list`, `details`, `update`, `unsubscribe`), customers (`create_customers`, `list_customers`, `customer_details`, `update_customer`), and `transactions`.
- `examples/src/bin/subscriptions.rs` demonstrating every `SubscriptionsAPI` method end-to-end, threading the returned `plan_id` / `customer_id` through subsequent calls.
- `FrequencyUnit` enum (`D`/`W`/`M`/`Y`) for subscription plan billing intervals.
- `SubscriptionsCustomer` response type matching the API `CustomerSer` schema, exported from the crate root.
- Re-exported the `rust_decimal` crate and its `Decimal` type from the crate root, so downstream users can build amounts with `intasend::Decimal::from(...)` / `intasend::Decimal::from_str(...)` without adding `rust_decimal` to their own `Cargo.toml`.
- Exported the subscriptions types and `Customer` from the crate root.

### Changed

- Subscription plan `amount` fields now use `rust_decimal::Decimal` instead of `String`, matching every other monetary field in the crate. The value (de)serializes as a decimal string on the wire (`#[serde(with = "rust_decimal::serde::str")]`) to honour the API contract (`^-?\d{0,13}(?:\.\d{0,2})?$`).
- Aligned the Subscriptions plan and customer types with the IntaSend OpenAPI schema: `SubscriptionsPlan` now carries `plan_id`, `plan_url`, `reference`, `redirect_url`, `created_at` and `updated_at`; subscription customers use the dedicated `SubscriptionsCustomer` type (`first_name`/`last_name`/`address`/`city`/`state`/`zipcode`/`country`/`reference`) instead of the generic `Customer`.
- `SubscriptionsCustomerCreateDetails` now takes required `first_name`/`last_name` plus optional `reference`/`address`/`city`/`state`/`zipcode`/`country` (was a guessed `phone_number`-based shape).

### Fixed

- Subscriptions module: corrected HTTP methods and endpoint paths (plans target `/api/v1/subscriptions-plans/`), fixed payload-dropping bugs in the customer create/update calls, gave the create/update methods proper payload parameters, and rewrote doc examples that incorrectly referenced the payment-links API.
- `amount` now decodes correctly when the API echoes it back as a JSON number (the sandbox returns a number even though the schema documents a string); it still serializes as a string on requests.
- `Subscription.subscription_id` is now a `String` (the API returns a short alphanumeric id, not a UUID), and `Subscription.reference` is now optional — both previously failed to decode real responses.
- `SubscriptionsTransactionListResponse` is now a transparent newtype over `Vec<Transaction>` because the transactions endpoint returns a bare JSON array rather than a paginated object.

### Removed

- Dropped the unused `rust_decimal_macros` dependency (the `dec!` macro is not used; amounts are constructed via `Decimal::from(...)`).

---

## 0.2.0

### Breaking Changes

- Updates:
  - Updated `crates.io` docs at doc.rs
  - Added struct identifiers
  - Renamed `Collections` struct to `CollectionsAPI`
  - Renamed `Checkout` struct to `CheckoutsAPI`
  - Updated error handling and legible error messgaes from the `IntaSend` Client 
  - Added GitHub Actions to automate the publishing of the crates to [crates.io](https://crates.io)
  - Added `docs` to support the library usage [here](https://github.com/MikeTeddyOmondi/intasend-rs/docs)
  - Version: `0.2.0` has `PayoutsAPI`, `RefundsAPI` and `WalletsAPI` support, bug fixes, refactored code

---

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
