use crate::authorization;
use crate::http;
use crate::pin;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;
use std::time::SystemTime;

pub const RELATIONSHIP_ACTION_ADD: &str = "ADD";
pub const RELATIONSHIP_ACTION_UPDATE: &str = "UPDATE";
pub const RELATIONSHIP_ACTION_REMOVE: &str = "REMOVE";
pub const RELATIONSHIP_ACTION_BLOCK: &str = "BLOCK";
pub const RELATIONSHIP_ACTION_UNBLOCK: &str = "UNBLOCK";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: String,
    identity_number: String,
    phone: String,
    full_name: String,
    biography: String,
    avatar_url: String,
    relationship: String,
    mute_until: String,
    created_at: String,
    is_verified: bool,
    is_scam: bool,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    #[serde(flatten)]
    user: User,
    session_id: String,
    pin_token_base64: String,
    code_id: String,
    code_url: String,
    device_status: String,
    has_pin: bool,
    has_emergency_contact: bool,
    receive_message_source: String,
    accept_conversation_source: String,
    accept_search_source: String,
    fiat_currency: String,
    transfer_notification_threshold: f64,
    transfer_confirmation_threshold: f64,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
}

pub fn me(cfg: authorization::AppConfig) -> Result<Me, Box<dyn error::Error>> {
    let map: HashMap<String, String> = HashMap::new();
    let res = http::request(cfg, Method::GET, "/me", &map)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<Me>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();

    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}

pub fn update(
    cfg: authorization::AppConfig,
    full_name: &str,
    avatar_base64: &str,
) -> Result<User, Box<dyn error::Error>> {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("full_name"), full_name.to_string());
    map.insert(String::from("avatar_base64"), avatar_base64.to_string());
    let res = http::request(cfg, Method::POST, "/me", &map)?;

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

pub fn code(cfg: authorization::AppConfig) -> Result<User, Box<dyn error::Error>> {
    let map: HashMap<String, String> = HashMap::new();
    let res = http::request(cfg, Method::GET, "/me/code", &map)?;

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

pub fn user(cfg: authorization::AppConfig, id: &str) -> Result<User, Box<dyn error::Error>> {
    let map: HashMap<String, String> = HashMap::new();
    let path = format!("/users/{}", id);
    let resp = http::request(cfg, Method::GET, &path, &map)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<User>,
        error: Option<http::Error>,
    }

    let body: Body = resp.json().unwrap();
    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}

pub fn fetch(
    cfg: authorization::AppConfig,
    ids: Vec<&str>,
) -> Result<Vec<User>, Box<dyn error::Error>> {
    let resp = http::request(cfg, Method::POST, "/users/fetch", &ids)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<Vec<User>>,
        error: Option<http::Error>,
    }

    let body: Body = resp.json().unwrap();
    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}

pub fn search(
    cfg: authorization::AppConfig,
    mixin_id: &str,
) -> Result<User, Box<dyn error::Error>> {
    let map: HashMap<String, String> = HashMap::new();
    let path = format!("/search/{}", mixin_id);
    let res = http::request(cfg, Method::GET, &path, &map)?;

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

#[derive(Debug, Serialize, Deserialize)]
struct PreferenceRequest {
    receive_message_source: String,
    accept_conversation_source: String,
    fiat_currency: String,
    transfer_notification_threshold: f64,
}

pub fn update_preference(
    cfg: authorization::AppConfig,
    message_source: &str,
    conversation_source: &str,
    currency: &str,
    threshold: f64,
) -> Result<User, Box<dyn error::Error>> {
    let preference: PreferenceRequest = PreferenceRequest {
        receive_message_source: message_source.to_string(),
        accept_conversation_source: conversation_source.to_string(),
        fiat_currency: currency.to_string(),
        transfer_notification_threshold: threshold,
    };
    let res = http::request(cfg, Method::POST, "/me", &preference)?;

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

pub fn relationship(
    cfg: authorization::AppConfig,
    user_id: &str,
    action: &str,
) -> Result<User, Box<dyn error::Error>> {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("user_id"), user_id.to_string());
    map.insert(String::from("action"), action.to_string());
    let res = http::request(cfg, Method::POST, "/relationships", &map)?;

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

pub fn update_pin(
    cfg: authorization::AppConfig,
    old: &str,
    fresh: &str,
) -> Result<Me, Box<dyn error::Error>> {
    let iterator = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let old_pin = pin::encrypt(old, iterator, &cfg.pin_token_base64, &cfg.private_base64)?;

    let fresh_pin = pin::encrypt(
        fresh,
        iterator + 1,
        &cfg.pin_token_base64,
        &cfg.private_base64,
    )?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("old_pin"), old_pin);
    map.insert(String::from("pin"), fresh_pin);
    let res = http::request(cfg, Method::POST, "/pin/update", &map)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<Me>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();

    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}

pub fn pin_verify(cfg: authorization::AppConfig) -> Result<Me, Box<dyn error::Error>> {
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
    let res = http::request(cfg, Method::POST, "/pin/verify", &map)?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<Me>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();

    match body.error {
        Some(e) => Err(Box::new(e)),
        None => Ok(body.data.unwrap()),
    }
}
