use super::Error;
use futures::prelude::*;
use log::*;
use std::convert::AsRef;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;
use tokio::net::TcpStream;
use tokio_serde::formats::Json;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use zerosumrs::ai::Ai;
use zerosumrs::default_heuristic::DefaultHeuristic;

use crate::common::{Action, State, Turn};
use crate::player::Player;

//use super::lines_codec::{mapc, Ai, Game, MinimaxSimple, State as GameState, Tablut, Tile};

#[derive(PartialEq)]
enum Role {
    White,
    Black,
}

#[derive(Error, Debug)]
#[error("expected player role to be one of : 'white', 'black', found {0}")]
pub struct ParseError(String);

impl FromStr for Role {
    type Err = ParseError;

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role.to_lowercase().as_ref() {
            "white" => Ok(Self::White),
            "black" => Ok(Self::Black),
            other => Err(ParseError(other.into())),
        }
    }
}

fn duration_from_secs(secs: &str) -> Result<Duration, std::num::ParseIntError> {
    Ok(Duration::from_secs(secs.parse::<u64>()? - 1))
}

#[derive(StructOpt)]
pub struct PlayerComm {
    /// Player name
    player_name: String,

    /// Player role (white/black)
    role: Role,

    /// Timeout for player move in seconds
    #[structopt(parse(try_from_str = duration_from_secs))]
    timeout: Duration,

    /// Server address
    server: SocketAddr,

    /// Which Ai to use
    player: Option<Player<DefaultHeuristic>>,
}

impl PlayerComm {
    pub async fn play(self) -> Result<(), Error> {
        let stream = TcpStream::connect(self.server).await?;
        println!("Connected to the server");

        // Prepend a header with frame length
        let mut new_line_delimited = Framed::new(stream, LengthDelimitedCodec::new());

        // send name
        new_line_delimited
            .send(bytes::Bytes::from(
                serde_json::to_vec(&serde_json::Value::String(self.player_name.clone())).unwrap(),
            ))
            .await?;

        // Serialize frames with JSON
        let mut comm_stream =
            tokio_serde::Framed::new(new_line_delimited, Json::<State, Action>::default());

        let mut turn = self.role == Role::White;
        let mut num_turns = 0;
        let mut player = self.player.unwrap_or_default();
        println!("Using player {}", player.as_ref());
        let outcome = loop {
            let new_state: State = comm_stream.next().await.expect("empty stream")?;
            if !matches!(new_state.turn, Turn::WHITE | Turn::BLACK) {
                break new_state.turn;
            }

            if num_turns > 0 {
                let mov = crate::util::mov_from_state(*player.get_game(), &new_state)?;
                player.mov(&mov);
            }

            if turn {
                println!("Calculating next move...");
                player.print2game();
                let mov = player.get_mov(self.timeout);
                comm_stream
                    .send(Action::from_move(mov, player.turn()))
                    .await?;
                println!("move sent...");
            } else {
                println!("waiting for adv...");
            }
            turn = !turn;
            num_turns += 1;
        };

        println!("game ended in {} turns", num_turns);
        match (outcome, self.role) {
            (Turn::WHITEWIN, Role::White) | (Turn::BLACKWIN, Role::Black) => {
                println!("clic click click submit world champion")
            }
            (Turn::DRAW, _) => println!("eh"),
            _ => println!("sad tablut noises"),
        }

        Ok(())
    }
}
