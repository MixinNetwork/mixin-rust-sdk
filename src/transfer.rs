use crate::authorization;
use crate::http;
use crate::pin;
use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OpponentMultisig {
    receivers: String,
    threshold: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransferRequest {
    asset_id: String,
    address_id: String,
    opponent_id: String,
    opponent_key: String,
    opponent_multisig: OpponentMultisig,
    amount: String,
    pin: String,
    trace_id: String,
    memo: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransferResponse {
    snapshot_id: String,
    opponent_id: String,
    asset_id: String,
    amount: String,
    opening_balance: String,
    closing_balance: String,
    trace_id: String,
    memo: String,
    created_at: chrono::DateTime<chrono::Utc>,

    transaction_hash: Option<String>,
    snapshot_hash: Option<String>,
    snapshot_at: Option<chrono::DateTime<chrono::Utc>>,
}

//pub fn transfer(cfg: authorization::AppConfig, in: TransferRequest) {}
