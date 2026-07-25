# Backlog

Tracking work toward a production-ready `intasend` crate. Items move from here into a versioned
section of [CHANGELOG.md](./CHANGELOG.md) once shipped.

## High priority

- [x] **Verify Subscriptions API against the live IntaSend reference.** The endpoint paths,
      HTTP methods, and request/response shapes for the Subscriptions API were implemented from
      convention; confirm each against <https://developers.intasend.com/reference> (especially
      `/api/v1/subscriptions/plans/`, `/api/v1/subscriptions-customers/`, and the unsubscribe /
      transactions sub-paths).
- [x] **Confirm `SubscriptionsCustomerCreateDetails` schema.** Fields (`email`, `first_name`,
      `last_name`, `phone_number`, `country`) are a reasonable guess — align with the real
      customer create payload.
- [ ] **Migrate date/time fields to a typed value (crate-wide).** `created_at` / `updated_at`
      (and similar) are `String` across every module. Move to a real date/time type (e.g.
      `chrono::DateTime` or `time::OffsetDateTime`) in a **backwards-compatible** way — e.g. a
      newtype that still (de)serializes from the current ISO-8601 string — so downstream code does
      not break. Subscriptions follows the existing `String` convention for now.
- [ ] **Confirm `amount` wire format for the remaining modules.** Subscriptions now serializes
      `amount` as a decimal string (per the API schema) via `rust_decimal::serde::str`. Verify
      whether the collection / payouts / wallets / checkout endpoints also expect strings (they
      currently serialize `Decimal` with serde's default) and apply the same adapter where needed.
- [ ] **Add integration/unit tests.** No tests exist yet. Add a mock HTTP server (e.g. `wiremock`)
      to cover the request building, serialization, and error paths for each API.
- [ ] **Confirm why the Subscription Checkout Links don't redirect.** Currently the `setup_url` doesn't redirect subscribing users to the checkout page.

## Medium priority

- [x] **Verify the subscription transactions item shape.** The transactions endpoint returns a bare
      JSON array; the sandbox returned it empty, so `SubscriptionsTransactionListResponse(Vec<Transaction>)` reuses the shared `Transaction` type unverified. Confirm the per-item fields against a populated response and add a dedicated type if they differ.
- [ ] **Documentation parity.** Add a Subscriptions chapter to the mdbook docs under `docs/src/`
      and wire it into `SUMMARY.md`, matching the other API chapters.
- [ ] **Error handling review.** Audit `IntasendClientError` / `IntasendApiError` for completeness
      and ensure all failure modes surface legible messages.
- [ ] **CI coverage.** Extend `.github/workflows/ci.yml` to build the `examples` crate and run
      doctests so regressions in the public API are caught.
- [ ] **Review `crate-type = ["cdylib", "rlib"]`.** Confirm the `cdylib` output is actually needed;
      drop it if the crate is consumed only as a Rust library.

## Low priority / nice-to-have

- [ ] Reduce `#![allow(unused)]` / `#![allow(unused_imports)]` usage once the API stabilizes.
- [ ] Consider `#![deny(missing_docs)]` for the public surface.
- [ ] Provide builder helpers or `Default` impls for the larger request structs.
