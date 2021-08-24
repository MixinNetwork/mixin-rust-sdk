use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod http;

pub fn hello() {
    println!("hello mixin");
}

#[derive(Debug, Serialize, Deserialize)]
struct Mixin {
    build: String,
    developers: String,
    test: Option<String>,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
    //timestamp: String,
}

pub fn root() {
    let res = http::client("")
        .get("https://mixin-api.zeromesh.net/")
        .send()
        .unwrap();

    #[derive(Debug, Serialize, Deserialize)]
    struct Body {
        data: Option<Mixin>,
        error: Option<http::Error>,
    }

    let body: Body = res.json().unwrap();

    match body.error {
        Some(e) => {
            println!("{:?}", e);
        }
        None => {
            println!("{:?}", body.data.unwrap());
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
