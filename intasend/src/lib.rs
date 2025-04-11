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

// Core types - available for both `client` and `server` environments
#[cfg(any(feature = "client", feature = "server"))]
pub use self::intasend::{
    Currency, Intasend, IntasendApiError, IntasendApiErrorDetail, IntasendClientError,
    PayoutProvider, Provider, Tarrif, Transaction, TransactionStatus, TransactionType,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Run tests here
    }
}
