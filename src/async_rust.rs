#![deny(clippy::all)]

use futures::executor::block_on;
use std::fmt::format;
use std::future::Future;
use tokio::time::{sleep, Duration};

async fn get_name() -> String {
    "John".to_string()
}

pub fn block_on_caller() {
    let name = block_on(get_name());
    println!("Hello, {}!", name)
}

// note you can use await syntax on async functions

pub async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "One".to_string()
}

pub async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "Two".to_string()
}

// async function don't have to use async fn
pub fn call_api_three() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Three".to_string()
    }
}

pub fn call_api_four() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;
        "Four".to_string()
    }
}

// Note: Asynchronous functions can move variables

pub fn get_async_name() -> impl Future<Output = String> {
    let name = "John Doe".to_string();
    async move { format!("Hello {} Doe", name).to_string() }
}
