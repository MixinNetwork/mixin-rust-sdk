use crate::authorization;
use crate::http;
use crate::pin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: String,
    session_id: String,
    pin_token_base64: String,
    identity_number: String,
    phone: String,
    full_name: String,
    avatar_url: String,
    device_status: String,
    created_at: String,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
    // timestamp: String,
}

pub fn me(cfg: authorization::AppConfig) -> Result<User, Box<dyn error::Error>> {
    let map: HashMap<String, String> = HashMap::new();
    let res = http::request(cfg, reqwest::Method::GET, "/me", &map)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<User>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();

    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}

pub fn pin_verify(mut cfg: authorization::AppConfig) -> Result<User, Box<dyn error::Error>> {
    let encrypted_pin = pin::encrypt(
        &cfg.pin,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
        &cfg.pin_token_base64,
        &cfg.private_base64,
    )?;
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("pin"), encrypted_pin);
    cfg.method = String::from("POST");
    cfg.uri = String::from("/pin/verify");
    let res = http::request(cfg, reqwest::Method::POST, "/pin/verify", &map)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<User>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();

    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}
