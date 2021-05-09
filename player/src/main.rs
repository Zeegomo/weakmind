mod client;
mod common;
mod player;
mod util;

use structopt::StructOpt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
    #[error("Could not map received state {0} to a new board")]
    StateDeserialize(String),
    #[error("Could not find a valid move for received state {0:?}")]
    InvalidNextState(Box<common::State>),
}

#[tokio::main]
async fn main() {
    stderrlog::new().init().unwrap();
    client::PlayerComm::from_args().play().await.unwrap();
}
