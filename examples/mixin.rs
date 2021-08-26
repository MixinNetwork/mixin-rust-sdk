mod secret;
use bot_api_rust_client::authorization;
use std::time::SystemTime;

fn generate_authorization_token() -> String {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        method: "GET".to_string(),
        uri: "/me".to_string(),
        body: "".to_string(),
    };
    bot_api_rust_client::authorization::sign_token(cfg).unwrap()
}

fn me() {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        method: "GET".to_string(),
        uri: "/me".to_string(),
        body: "".to_string(),
    };
    let user = bot_api_rust_client::user::me(cfg).unwrap();
    println!("{:?}", user);
}

fn main() {
    //bot_api_rust_client::root();
    //println!("token {}", generate_authorization_token())
    //println!(
    //    "{}",
    //    bot_api_rust_client::pin::encrypt(
    //        secret::PIN,
    //        SystemTime::now()
    //            .duration_since(SystemTime::UNIX_EPOCH)
    //            .unwrap()
    //            .as_secs(),
    //        secret::PIN_TOKEN,
    //        secret::PRIVATE_KEY,
    //    )
    //    .unwrap()
    //);

    me();
}
