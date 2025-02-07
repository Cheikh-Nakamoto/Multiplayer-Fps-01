use multiplayer_fps::*;
use std::io::{self, Error};
#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    run_server().await
}
