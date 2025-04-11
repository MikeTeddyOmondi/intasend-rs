#![allow(unused)]
#![allow(unused_imports)]

use anyhow::{Error, Result};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{Intasend, Wallet};

use super::{Currency, PayoutProvider, RequestClient, RequestMethods};

/// `PayoutsAPI` struct implements methods for facilitating:
/// Sending of funds to different recipients programatically.
///
/// ```rust
/// // Load .env file
/// dotenvy::dotenv().ok();
///
/// let intasend_public_key = std::env::var("INTASEND_PUBLIC_KEY").expect("INTASEND_PUBLIC_KEY must be set");
/// let intasend_secret_key = std::env::var("INTASEND_SECRET_KEY").expect("INTASEND_SECRET_KEY must be set");
///
/// // Intasend Client
/// let intasend = intasend::Intasend::new(
///    intasend_public_key,
///    intasend_secret_key,
///     true,
/// );
///
/// // PayoutsAPI
/// let payout: intasend::PayoutsAPI = intasend.payouts();
/// ```
///
#[derive(Deserialize, Debug)]
pub struct PayoutsAPI {
    pub(crate) intasend: Intasend,
}

impl PayoutsAPI {
    /// The `initiate` method initiates the API requests to IntaSend's `Payouts` (Send Money) API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments.
    /// This method is used under the hood to power all initiated requests depending on PayoutProvider.
    /// It returns a Result of Payout.
    pub async fn initiate(&self, payload: PayoutRequest) -> Result<Payout> {
        let service_path: &str = "/api/v1/send-money/initiate/";
        let request_method = RequestMethods::Post;

        let payout = self
            .intasend
            .send::<PayoutRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    /// The `mpesa_b2c` method initates Mpesa B2C payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn mpesa_b2c(&self, payload: PayoutRequest) -> Result<Payout> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::MpesaB2c);
        let mpesa_payouts = self.initiate(payload).await?;
        Ok(mpesa_payouts)
    }

    /// The `mpesa_b2b`  method initates Mpesa B2B payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn mpesa_b2b(&self, payload: PayoutRequest) -> Result<Payout> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::MpesaB2b);
        let mpesa_b2b = self.initiate(payload).await?;
        Ok(mpesa_b2b)
    }

    /// The `bank` method initates bank payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn bank(&self, payload: PayoutRequest) -> Result<Payout> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::Pesalink);
        let bank_payout = self.initiate(payload).await?;
        Ok(bank_payout)
    }

    /// The `intasend` method initiates intasend payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments.
    /// It returns a Payout Result.
    pub async fn intasend(&self, payload: PayoutRequest) -> Result<Payout> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::Intasend);
        let intasend_payout = self.initiate(payload).await?;
        Ok(intasend_payout)
    }

    /// The `airtime` method initiates airtime payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn airtime(&self, payload: PayoutRequest) -> Result<Payout> {
        let mut payload = payload;
        payload.provider = Some(PayoutProvider::Airtime);
        let airtime = self.initiate(payload).await?;
        Ok(airtime)
    }

    /// The `approve` method approves the initiated payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutApprovalRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn approve(&self, payload: PayoutApprovalRequest) -> Result<Payout> {
        let service_path: &str = "/api/v1/send-money/approve/";
        let request_method = RequestMethods::Post;

        let payout = self
            .intasend
            .send::<PayoutApprovalRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    /// The `status` method checks the status of the initiated payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutStatusRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn status(&self, payload: PayoutStatusRequest) -> Result<Payout> {
        let service_path: &str = "/api/v1/send-money/status/";
        let request_method = RequestMethods::Post;

        let payout = self
            .intasend
            .send::<PayoutStatusRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    /// The `cancel` method cancels the initiated payout API requests to IntaSend's `Payouts` API.
    /// This depends on the payload struct (`PayoutStatusRequest`) passed into the method as arguments.
    /// It returns a Result of Payout.
    pub async fn cancel(&self, payload: PayoutCancelRequest) -> Result<Payout> {
        let service_path: &str = "/api/v1/send-money/cancel/";
        let request_method = RequestMethods::Post;

        let payout = self
            .intasend
            .send::<PayoutCancelRequest, Payout>(Some(payload), service_path, request_method)
            .await?;

        Ok(payout)
    }

    /// The `bank_codes_ke` method gets all bank codes from the IntaSend's API.
    /// This depends on the payload struct (`PayoutStatusRequest`) passed into the method as arguments
    /// It returns a Result of Vec of `BankCodes`
    pub async fn bank_codes_ke(&self) -> Result<Vec<BankCodes>> {
        let service_path: &str = "/api/v1/send-money/bank-codes/ke/";
        let request_method = RequestMethods::Get;

        let payout = self
            .intasend
            .send_client_request::<PayoutBankCodesRequest, Vec<BankCodes>>(
                None,
                service_path,
                request_method,
            )
            .await?;

        Ok(payout)
    }
}

/// `Payout` struct
#[derive(Deserialize, Serialize, Debug)]
pub struct Payout {
    pub file_id: Option<String>,
    pub device_id: Option<String>,
    pub tracking_id: Option<String>,
    pub batch_reference: Option<String>,
    pub status: Option<String>,
    pub status_code: Option<String>,
    pub nonce: Option<String>,
    pub wallet: Option<Wallet>,
    pub transactions: Option<Vec<PayoutResponseTransaction>>,
    pub charge_estimate: Option<Decimal>,
    pub total_amount_estimate: Option<Decimal>,
    pub total_amount: Option<Decimal>,
    pub transactions_count: Option<u32>,
}

/// `PayoutRequestTransaction` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutRequestTransaction {
    /// Beneficiary name as per Client Records
    pub name: Option<String>,
    /// Phone number, bank account number etc
    pub account: String,
    /// Optional ID number of beneficiary - M-pesa transaction will be validated
    pub id_number: Option<String>,
    pub amount: Decimal,
    pub bank_code: Option<String>,
    pub category_name: Option<String>,
    pub narrative: Option<String>,
    pub account_type: Option<String>,
    pub account_reference: Option<String>,
}

/// `PayoutResponseTransaction` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PayoutResponseTransaction {
    pub status: Option<String>,
    pub status_code: Option<String>,
    pub request_reference_id: Option<String>,
    /// Beneficiary name as per Client Records
    pub name: Option<String>,
    /// Phone number, bank account number etc
    pub account: String,
    /// Optional ID number of beneficiary - M-pesa transaction will be validated
    pub id_number: Option<String>,
    pub bank_code: Option<String>,
    pub amount: Decimal,
    pub narrative: Option<String>,
}

/// `PayoutRequest` struct
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayoutRequest {
    pub currency: Currency,
    pub provider: Option<PayoutProvider>, // PayoutProvider,
    pub device_id: Option<String>,
    pub callback_url: Option<String>,
    pub batch_reference: Option<String>,
    //pub requires_approval: PayoutApproval,
    pub transactions: Vec<PayoutRequestTransaction>,
}

/// `PayoutApprovalRequest` struct
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayoutApprovalRequest {
    pub tracking_id: String,
    pub batch_reference: String,
    pub nonce: String,
    pub wallet: Option<Wallet>,
    pub transactions: Option<Vec<PayoutResponseTransaction>>,
}

/// `PayoutStatusRequest` struct
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayoutStatusRequest {
    pub tracking_id: String,
}

/// `PayoutCancelRequest` struct
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayoutCancelRequest {
    pub file_id: String,
}

/// `PayoutBankCodesRequest` struct
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PayoutBankCodesRequest {}

/// `PayoutApproval` enum
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PayoutApproval {
    Yes,
    No,
}

/// `BankCodes` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BankCodes {
    bank_name: String,
    bank_code: String,
}
