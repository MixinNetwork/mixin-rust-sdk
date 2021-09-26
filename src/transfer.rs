use crate::authorization;
use crate::http;
use crate::pin;
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpponentMultisig {
    pub receivers: String,
    pub threshold: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub asset_id: String,

    // transfer
    pub opponent_id: String,
    pub amount: String,
    pub pin: String,
    pub trace_id: String,
    pub memo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    snapshot_id: String,
    opponent_id: String,
    asset_id: String,
    amount: String,
    opening_balance: String,
    closing_balance: String,
    trace_id: String,
    memo: String,
    created_at: chrono::DateTime<chrono::Utc>,

    snapshot_hash: Option<String>,
    transaction_hash: Option<String>,
    snapshot_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
}

pub fn transfer(
    cfg: authorization::AppConfig,
    mut input: TransferRequest,
) -> Result<TransferResponse, Box<dyn error::Error>> {
    let encrypted_pin = pin::encrypt(
        &cfg.pin,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
        &cfg.pin_token_base64,
        &cfg.private_base64,
    )?;
    input.pin = encrypted_pin;

    let res = http::request(cfg, reqwest::Method::POST, "/transfers", &input)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<TransferResponse>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();
    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}
