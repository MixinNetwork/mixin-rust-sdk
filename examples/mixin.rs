mod secret;
use std::time::SystemTime;

fn generate_authorization_token() -> String {
    bot_api_rust_client::authorization::sign_token(
        secret::APP_ID,
        secret::SESSION_ID,
        secret::PRIVATE_KEY,
        "GET",
        "/me",
        "",
    )
    .unwrap()
}

fn main() {
    //bot_api_rust_client::root();
    //println!("token {}", generate_authorization_token())
    println!(
        "{}",
        bot_api_rust_client::pin::encrypt(
            secret::PIN,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            secret::PIN_TOKEN,
            secret::PRIVATE_KEY,
        )
        .unwrap()
    );
}
