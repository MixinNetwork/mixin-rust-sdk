mod secret;
use bot_api_rust_client::{authorization, transfer};
use reqwest::Method;
use uuid::Uuid;

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

fn user() {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        pin: "".to_string(),
        pin_token_base64: "".to_string(),
    };
    let user =
        bot_api_rust_client::user::read(cfg, "e9e5b807-fa8b-455a-8dfa-b189d28310ff").unwrap();
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

fn transfer() {
    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        pin: secret::PIN.to_string(),
        pin_token_base64: secret::PIN_TOKEN.to_string(),
    };

    let mut input: transfer::TransferRequest = transfer::TransferRequest {
        asset_id: "965e5c6e-434c-3fa9-b780-c50f43cd955c".to_string(),
        opponent_id: "e9e5b807-fa8b-455a-8dfa-b189d28310ff".to_string(),
        amount: "0.000001".to_string(),
        pin: "".to_string(),
        trace_id: Uuid::new_v4().to_string(),
        memo: "hello from rust".to_string(),
    };
    let transfer = bot_api_rust_client::transfer::transfer(cfg, input).unwrap();
    println!("{:?}", transfer);
}

fn main() {
    /*
    bot_api_rust_client::root();
    println!("token {}", generate_authorization_token());
    println!(
        "{}",
        bot_api_rust_client::pin::encrypt(
            secret::PIN,
     LittleEndian::write_u64(
         &mut time_buf,
         SystemTime::now()
             .duration_since(SystemTime::UNIX_EPOCH)?
             .as_secs(),
     );
            secret::PIN_TOKEN,
            secret::PRIVATE_KEY,
        )
        .unwrap()
    );

    me();
    user();
    verify_pin();
    */
    transfer();
}
