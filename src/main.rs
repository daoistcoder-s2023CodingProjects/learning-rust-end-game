#![deny(clippy::all)]

use crate::async_rust::{
    block_on_caller, call_api_four, call_api_one, call_api_three, call_api_two,
};

mod async_rust;

#[tokio::main]
async fn main() {
    // block_on_caller();

    let one = call_api_one().await;
    println!("{}", one);

    let two = call_api_two().await;
    println!("{}", two);

    let three = call_api_three().await;
    println!("{}", three);

    let four = call_api_four().await;
    println!("{}", four);
}
