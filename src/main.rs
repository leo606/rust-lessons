#![deny(clippy::all)]

use tokio::time::{sleep, Duration};

async fn call_api_1() -> String {
    sleep(Duration::from_secs(3)).await;
    "John".to_string()
}

#[tokio::main]
async fn main() {
    let name = call_api_1().await;
    println!("{}", name)
}
