use jwt_simple::claims::JWTClaims;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct MyAdditionalData {
    uid: String,
    sid: String,
    iat: u64,
    exp: u64,
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

    let expired = SystemTime::now() + Duration::new(60 * 60 * 24 * 30 * 6, 0);

    let private_data = base64::decode_config(private, base64::URL_SAFE_NO_PAD)?;

    let payload = MyAdditionalData {
        uid: uid.to_string(),
        sid: sid.to_string(),
        iat: SystemTime::now().duration_since(UNIX_EPOCH)?,
        exp: expired.duration_since(UNIX_EPOCH)?,
        jti: Uuid::new_v4().to_string(),
        sig: format!("{:x}", result),
        scp: "FULL".to_owned(),
    };
    let claims = Claims::with_custom_claims(payload, jwt_duration::from_hours(2)); // TODO

    let key_pair = Edwards25519KeyPair::from_bytes(private_data)?;
    key_pair.sign(claims)?
}
