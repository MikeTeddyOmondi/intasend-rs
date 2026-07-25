#![allow(unused_imports)]

mod intasend;

// Checkout functionality - available for both `client` and `server` environments
#[cfg(any(feature = "client", feature = "server"))]
pub use self::intasend::checkout::{
    CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse, CheckoutsAPI,
};

// Collection functionality - `server` only
#[cfg(feature = "server")]
pub use self::intasend::collection::{
    CollectionsAPI, MpesaStkPushRequest, MpesaStkPushResponse, StkPushStatusRequest,
    StkPushStatusResponse,
};

// Payout functionality - `server` only
#[cfg(feature = "server")]
pub use self::intasend::payouts::{
    Payout, PayoutApproval, PayoutApprovalRequest, PayoutRequest, PayoutRequestTransaction,
    PayoutResponseTransaction, PayoutStatusRequest, PayoutsAPI,
};

// Refund functionality - `server` only
#[cfg(feature = "server")]
pub use self::intasend::refunds::{
    Refund, RefundListResponse, RefundReason, RefundRequest, RefundsAPI,
};

// Wallet functionality - `server` only
#[cfg(feature = "server")]
pub use self::intasend::wallets::{
    FundCheckoutRequest, FundCheckoutResponse, FundMpesaRequest, FundMpesaResponse, Wallet,
    WalletCreateDetails, WalletDetailsRequest, WalletIntraTransferRequest,
    WalletIntraTransferResponse, WalletListResponse, WalletTransactionsResponse, WalletType,
    WalletsAPI,
};

// Payment Links functionality - `server` only
#[cfg(feature = "server")]
pub use self::intasend::payment_links::{
    PaymentLink, PaymentLinksAPI, PaymentLinksCreateDetails, PaymentLinksDetailsRequest,
    PaymentLinksListRequest, PaymentLinksListResponse, PaymentLinksUpdateDetails,
};

// Subscription functionality - `server` only
#[cfg(feature = "server")]
pub use self::intasend::subscriptions::{
    FrequencyUnit, Subscription, SubscriptionStatus, SubscriptionsAPI, SubscriptionsCreateDetails,
    SubscriptionsCustomer, SubscriptionsCustomerCreateDetails, SubscriptionsCustomerListResponse,
    SubscriptionsListResponse, SubscriptionsPlan, SubscriptionsPlanCreateRequest,
    SubscriptionsPlanListResponse, SubscriptionsTransactionListResponse,
};

// Core types - available for both `client` and `server` environments
#[cfg(any(feature = "client", feature = "server"))]
pub use self::intasend::{
    Currency, Customer, Intasend, IntasendApiError, IntasendApiErrorDetail, IntasendClientError,
    PayoutProvider, Provider, Tarrif, Transaction, TransactionStatus, TransactionType,
};

// Money type re-exports. `Decimal` is used for every monetary `amount`/balance field in this
// crate's request and response types. We re-export the whole `rust_decimal` crate as-is (so
// `intasend::rust_decimal::...` is available) plus the `Decimal` type directly, letting
// downstream users construct amounts with `intasend::Decimal::from(1000)`,
// `intasend::Decimal::from_str("10.00")`, etc. without adding `rust_decimal` to their own
// `Cargo.toml`. This is additive: existing users importing `rust_decimal::Decimal` directly
// are unaffected.
#[cfg(any(feature = "client", feature = "server"))]
pub use rust_decimal::{self, Decimal};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Run tests here
    }
}
