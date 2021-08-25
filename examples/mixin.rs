use std::time::{Duration, SystemTime};

fn main() {
    let expired = SystemTime::now() + Duration::new(60 * 60 * 24 * 30 * 6, 0);
    println!("expired {:?}", expired);
    //bot_api_rust_client::root();
}
