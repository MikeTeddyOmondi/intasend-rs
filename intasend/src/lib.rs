#![allow(unused_imports)]

mod intasend;

pub use self::intasend::checkout::{
    Checkout, CheckoutDetailsRequest, CheckoutDetailsResponse, CheckoutMethod, CheckoutRequest,
    CheckoutResponse,
};
pub use self::intasend::collection::{
    Collection, MpesaStkPushRequest, MpesaStkPushResponse, StkPushStatusRequest, StkPushStatusResponse,
};
pub use self::intasend::payouts::{Payout, PayoutRequest, Payouts};
pub use self::intasend::refunds::{RefundRequest, Refunds};
pub use self::intasend::wallets::{FundCheckoutRequest, Transaction, Wallet, Wallets};
pub use self::intasend::{Currency, Intasend};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Run tests here
    }
}
