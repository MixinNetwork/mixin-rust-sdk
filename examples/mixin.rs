mod secret;
use bot_api_rust_client::authorization;
use reqwest::Method;
use std::time::SystemTime;

fn generate_authorization_token() -> String {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        pin: "".to_string(),
        pin_token_base64: "".to_string(),
    };
    bot_api_rust_client::authorization::sign_token(Method::GET, "/me", "", cfg).unwrap()
}

fn me() {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        pin: "".to_string(),
        pin_token_base64: "".to_string(),
    };
    let user = bot_api_rust_client::user::me(cfg).unwrap();
    println!("{:?}", user);
}

fn verify_pin() {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        pin: secret::PIN.to_string(),
        pin_token_base64: secret::PIN_TOKEN.to_string(),
    };
    let user = bot_api_rust_client::user::pin_verify(cfg).unwrap();
    println!("{:?}", user);
}

fn main() {
    //bot_api_rust_client::root();
    //println!("token {}", generate_authorization_token())
    //println!(
    //    "{}",
    //    bot_api_rust_client::pin::encrypt(
    //        secret::PIN,
    // LittleEndian::write_u64(
    //     &mut time_buf,
    //     SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)?
    //         .as_secs(),
    // );
    //        secret::PIN_TOKEN,
    //        secret::PRIVATE_KEY,
    //    )
    //    .unwrap()
    //);

    // me();
    verify_pin();
}
