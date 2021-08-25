use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct MyAdditionalData {
    uid: String,
    sid: String,
    jti: String,
    sig: String,
    scp: String,
}

pub fn sign_authorization_token(
    uid: &str,
    sid: &str,
    private: &str,
    method: &str,
    uri: &str,
    body: &str,
) -> Result<String, Box<dyn error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}", method, uri, body).as_bytes());
    let result = hasher.finalize();

    let private_data = base64::decode_config(private, base64::URL_SAFE_NO_PAD)?;

    let payload = MyAdditionalData {
        uid: uid.to_string(),
        sid: sid.to_string(),
        jti: Uuid::new_v4().to_string(),
        sig: format!("{:x}", result),
        scp: "FULL".to_owned(),
    };
    let claims = Claims::with_custom_claims(payload, Duration::from_hours(24 * 30 * 6));

    let key_pair = Ed25519KeyPair::from_bytes(private_data.as_slice())?;
    Ok(key_pair.sign(claims)?)
}
