#![deny(clippy::all)]

use futures::Future;
use tokio::time::{sleep, Duration};

fn call_api_1() -> impl Future<Output = String> {
    async {
        // its a good idea create all variables inside the async clock
        sleep(Duration::from_secs(3)).await;

        // stored in the heap
        // it lives on for as long as the async block lives on
        "one".to_string()

    }
}

async fn call_api_2() -> String {
    sleep(Duration::from_secs(3)).await;
    "two".to_string()
}

#[tokio::main]
async fn main() {
    let name = call_api_1().await;
    println!("{}", name);

    let name2 = call_api_2().await;
    println!("{}", name2)
}
