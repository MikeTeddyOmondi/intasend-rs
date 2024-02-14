#![allow(unused_imports)]

mod intasend;

pub use self::intasend::collection::{
    Collection, MpesaStkPushRequest, MpesaStkPushResponse,
    StatusRequest, StatusResponse,
};
pub use self::intasend::checkout::{Checkout, CheckoutRequest, CheckoutResponse, CheckoutDetailsResponse, CheckoutMethod};
pub use self::intasend::payouts::{Payout, PayoutRequest, Payouts};
pub use self::intasend::refunds::{RefundRequest, Refunds};
pub use self::intasend::wallets::{FundCheckoutRequest, Transaction, Wallet, Wallets};
pub use self::intasend::{Intasend, Currency};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Run tests here
    }
}
