use jwt_simple::prelude::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct CustomClaims {
    uid: String,
    sid: String,
    jti: String,
    sig: String,
    scp: String,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub uid: String,
    pub sid: String,
    pub private_base64: String,
    pub pin: String,
    pub pin_token_base64: String,
}

pub fn sign_token(
    method: Method,
    uri: &str,
    body: &str,
    cfg: AppConfig,
) -> Result<String, Box<dyn error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}", method.as_str(), uri, body).as_bytes());
    let result = hasher.finalize();

    let private_data = base64::decode_config(cfg.private_base64, base64::URL_SAFE_NO_PAD)?;

    let claim = CustomClaims {
        uid: cfg.uid.to_string(),
        sid: cfg.sid.to_string(),
        jti: Uuid::new_v4().to_string(),
        sig: format!("{:x}", result),
        scp: "FULL".to_owned(),
    };
    let claims = Claims::with_custom_claims(claim, Duration::from_hours(24 * 30 * 6));

    let key_pair = Ed25519KeyPair::from_bytes(private_data.as_slice())?;
    Ok(key_pair.sign(claims)?)
}
