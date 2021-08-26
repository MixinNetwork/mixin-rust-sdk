use crate::authorization;
use crate::http;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;

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
    let res = http::request(cfg, reqwest::Method::GET, "/me")?;

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
