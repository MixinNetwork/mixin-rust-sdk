mod secret;
use bot_api_rust_client::{authorization, transfer, user};
use reqwest::Method;
use uuid::Uuid;
use oauth2::url::Url;

use oauth2::basic::BasicClient;

use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::collections::HashMap;

fn main() {

    let cfg: authorization::AppConfig = authorization::AppConfig {
        uid: secret::APP_ID.to_string(),
        sid: secret::SESSION_ID.to_string(),
        private_base64: secret::PRIVATE_KEY.to_string(),
        pin: secret::PIN.to_string(),
        pin_token_base64: secret::PIN_TOKEN.to_string(),
    };

    let client_id = ClientId::new(cfg.uid.to_string());
    let client_secret = ClientSecret::new(cfg.private_base64.to_string());
    let auth_url = AuthUrl::new("https://www.mixin.one/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.mixin.one/oauth/token".to_string())
        .expect("Invalid token endpoint URL");
    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    ).set_redirect_uri(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    );

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("PROFILE:READ".to_string()))
        .add_scope(Scope::new("ASSETS:READ".to_string()))
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Mixin returned the following code:\n{}\n", code.secret());
            println!(
                "Mixin returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_state.secret()
            );

	    let mut payload = HashMap::new(); 
	    payload.insert("client_id", cfg.uid.to_string());
	    payload.insert("code", code.secret().to_string());
	    payload.insert("client_secret", cfg.private_base64.to_string());
	    let mut buffer = std::fs::File::create("body.json").unwrap();
	    writeln!(buffer, "{:?}", &payload);

            // Exchange the code with a token.
            let token_res = client.exchange_code(code).request(http_client);

            println!("Mixin returned the following token:\n{:#?}\n", token_res);

            if let Ok(token) = token_res {
                let scopes = if let Some(scopes_vec) = token.scopes() {
                    scopes_vec
                        .iter()
                        .map(|comma_separated| comma_separated.split(','))
                        .flatten()
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                println!("Mixin returned the following scopes:\n{:?}\n", scopes);
            }

            // The server will terminate itself after collecting the first code.
            break;
        }
    }
}