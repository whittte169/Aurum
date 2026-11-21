use std::io;
use tokio;

mod system;
mod network;
mod ui;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    Ok(())
}