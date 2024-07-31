#![allow(unused_imports)]

mod intasend;

pub use self::intasend::checkout::{
    CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse, CheckoutsAPI,
};
pub use self::intasend::collection::{
    CollectionsAPI, MpesaStkPushRequest, MpesaStkPushResponse, StkPushStatusRequest,
    StkPushStatusResponse,
};
pub use self::intasend::payouts::{
    Payout, PayoutApproval, PayoutRequest, PayoutTransaction, PayoutsAPI,
};
pub use self::intasend::refunds::{
    Refund, RefundReason, RefundRequest, RefundResponse, RefundsAPI,
};
pub use self::intasend::wallets::{
    FundCheckoutRequest, Wallet, WalletCreateDetails, WalletDetailsRequest,
    WalletIntraTransferRequest, WalletIntraTransferResponse, WalletListResponse,
    WalletTransactionsResponse, WalletType, WalletsAPI,
};
pub use self::intasend::{Currency, Intasend, PayoutProvider, Transaction};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Run tests here
    }
}
