use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use zerosumrs::game::{Game, State as GameState};
use zerosumrs::tablut::{Tablut, Tile};

#[derive(Serialize, Debug)]
pub struct Action {
    pub from: String,
    pub to: String,
    // useless, but this is included in the server message
    _turn: Turn,
}

impl Action {
    pub fn from_move(mov: (u8, u8), turn: bool) -> Self {
        let from = format!("{}{}", (mov.0 % 11 + 97 - 1) as char, mov.0 / 11);
        let to = format!("{}{}", (mov.1 % 11 + 97 - 1) as char, mov.1 / 11);
        let _turn = if turn { Turn::WHITE } else { Turn::BLACK };
        Action { from, to, _turn }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone, Default)]
pub struct State {
    pub board: [[String; 9]; 9],
    pub turn: Turn,
}

#[derive(Debug, PartialEq, EnumString, Serialize, Deserialize, Clone)]
pub enum Turn {
    WHITEWIN,
    BLACKWIN,
    DRAW,
    WHITE,
    BLACK,
}

impl Default for Turn {
    fn default() -> Self {
        Turn::WHITE
    }
}

impl From<&Tablut> for State {
    fn from(game: &Tablut) -> Self {
        let turn = match (game.turn(), game.state()) {
            (_, GameState::Win) => Turn::WHITEWIN,
            (_, GameState::Lose) => Turn::BLACKWIN,
            (_, GameState::Draw) => Turn::DRAW,
            (true, _) => Turn::WHITE,
            (false, _) => Turn::BLACK,
        };
        let mut state = State {
            turn,
            ..Default::default()
        };
        let board = game.get_board();

        for y in 0..9 {
            for x in 0..9 {
                state.board[y][x] = match board[y][x] {
                    Tile::K => "KING".into(),
                    Tile::A => "BLACK".into(),
                    Tile::D => "WHITE".into(),
                    Tile::E if x == 4 && y == 4 => "THRONE".into(),
                    Tile::E => "EMPTY".into(),
                }
            }
        }

        state
    }
}
