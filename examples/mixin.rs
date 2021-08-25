mod secret;

fn main() {
    //bot_api_rust_client::root();
    println!(
        "token {}",
        bot_api_rust_client::authorization::sign_authorization_token(
            secret::APP_ID,
            secret::SESSION_ID,
            secret::PRIVATE_KEY,
            "GET",
            "/me",
            "",
        )
        .unwrap()
    )
}
