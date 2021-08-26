use crate::authorization;
use reqwest::{blocking::Response, header, Method};
use serde::{Deserialize, Serialize};
use std::{error, fmt, time::Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    status: u32,
    code: u32,
    description: String,
    #[serde(default)]
    extra: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl error::Error for Error {}

pub fn request(
    cfg: authorization::AppConfig,
    method: Method,
    path: &str,
) -> Result<Response, Box<dyn error::Error>> {
    let token = authorization::sign_token(cfg)?;

    Ok(request_with_token(method, path, &token))
}

pub fn request_with_token(method: Method, path: &str, token: &str) -> Response {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Client::build()");

    let uri = format!("https://mixin-api.zeromesh.net{}", path).to_string();
    client.request(method, uri).send().unwrap()
}
