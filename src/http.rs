use crate::authorization;
use reqwest::{blocking::Response, header, Method};
use serde::{Deserialize, Serialize};
use std::{error, fmt, time::Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub status: u32,
    pub code: u32,
    pub description: String,

    #[serde(default)]
    pub extra: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "status: {}, code: {}, description: {}, extra: {}",
            self.status, self.code, self.description, self.extra
        )
    }
}

impl error::Error for Error {}

pub fn request<T: Serialize + ?Sized>(
    cfg: authorization::AppConfig,
    method: Method,
    path: &str,
    json: &T,
) -> Result<Response, Box<dyn error::Error>> {
    let mut body = String::from("");
    if method == "POST" {
        let j = serde_json::to_string(&json)?;
        body = j.clone();
    }
    let token = authorization::sign_token(method.clone(), path, &body, cfg)?;

    Ok(request_with_token(method, path, json, &token))
}

pub fn request_with_token<T: Serialize + ?Sized>(
    method: Method,
    path: &str,
    json: &T,
    token: &str,
) -> Response {
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
    if method == Method::GET {
        return client.request(method, uri).send().unwrap();
    }

    client.request(method, uri).json(json).send().unwrap()
}
