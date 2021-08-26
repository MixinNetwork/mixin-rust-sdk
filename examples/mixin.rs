mod secret;

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
    bot_api_rust_client::pin::encrypt(
        "",
        "bvDhJuoHZfAU4UjMwKHkqUVxf0n_-nwM2YCQf9hOYAA",
        "",
        "QyyX3LAsCvW75c5L-N6eSmH-WsUhIN47nr0t5Libcy4",
        0,
    );
}
