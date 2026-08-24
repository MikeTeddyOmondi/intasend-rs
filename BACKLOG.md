# Backlog

Tracking work toward a production-ready `intasend` crate. Items move from here into a versioned
section of [CHANGELOG.md](./CHANGELOG.md) once shipped.

## High priority

- [x] **`Currency` now covers the Subscriptions set.** Was `KES`, `USD`, `EUR`,
      `GBP`; the plan endpoint accepts `KES`, `GHS`, `NGN`, `UGX`, `TZS`,
      `XAF`, `XOF`, so six currencies could not be expressed at all — Ghana,
      Nigeria, Uganda, Tanzania and both CFA zones.

      Merged into the shared enum rather than a subscriptions-specific type: a
      currency is a currency, and two enums with overlapping variants push the
      conversion onto every caller. Which endpoint accepts which is documented
      on the variants, and the API remains the authority.

      Covered by tests that pin each variant's wire value — a renamed variant
      is a rejected request and nothing in Rust would catch it.

- [x] **Endpoint paths confirmed against the reference index.**
      `/api/v1/subscriptions-plans/`, `/api/v1/subscriptions-customers/` and
      `/api/v1/subscriptions/` all match; the hyphenated forms were right and
      the `subscriptions/plans/` shape suspected earlier was not.

- [x] **Every path verified against the OpenAPI spec.** Appending `.md` to any
      reference URL returns the page with its full OpenAPI definition inline —
      no browser needed, and the authoritative source:

      ```
      POST /api/v1/subscriptions-plans/            ✓
      POST /api/v1/subscriptions/{id}/unsubscribe/ ✓
      GET  /api/v1/subscriptions/{id}/transactions/ ✓
      ```

      All three match what the crate already sends. The paths written "from
      convention" were right.

- [ ] **`frequency_unit` and `billing_cycles` are required in the crate and
      optional in the spec.** Neither appears in the plan schema's `required`
      list, yet both are non-`Option` on `SubscriptionsPlanCreateRequest`, so a
      caller cannot omit them.

      0.4.0 is unpublished, so this is the moment to change it — after a release
      it becomes a breaking change for anyone constructing the struct.

- [ ] **`billing_cycles` is required in the request type and optional in the
      API.** `SubscriptionsPlanCreateRequest.billing_cycles` is `u32`; the
      reference does not mark it required, and the response type already has it
      as `Option<u32>`. A caller who wants an open-ended plan cannot express
      one.

- [ ] **Nothing has been run against the sandbox.** The shapes below were
      checked against the published reference, which is not the same as a
      request that succeeded. Until a plan is created, a customer subscribed and
      a subscription cancelled against sandbox credentials, treat the whole
      Subscriptions module as unverified.

      The two checks most likely to fail are the ones no reading can settle:
      whether `amount` really wants a decimal string in practice, and whether
      the unsubscribe and transactions sub-paths are where convention suggests.

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
