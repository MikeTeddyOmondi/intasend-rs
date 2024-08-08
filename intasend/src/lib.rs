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
    Payout, PayoutApproval, PayoutApprovalRequest, PayoutRequest, PayoutRequestTransaction,
    PayoutResponseTransaction, PayoutStatusRequest, PayoutsAPI,
};
pub use self::intasend::refunds::{
    Refund, RefundListResponse, RefundReason, RefundRequest, RefundsAPI,
};
pub use self::intasend::wallets::{
    FundCheckoutRequest, FundCheckoutResponse, FundMpesaRequest, FundMpesaResponse, Wallet,
    WalletCreateDetails, WalletDetailsRequest, WalletIntraTransferRequest,
    WalletIntraTransferResponse, WalletListResponse, WalletTransactionsResponse, WalletType,
    WalletsAPI,
};
pub use self::intasend::{
    Currency, Intasend, PayoutProvider, Provider, Tarrif, Transaction, TransactionStatus,
    TransactionType,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Run tests here
    }
}
