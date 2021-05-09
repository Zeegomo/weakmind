use std::str::FromStr;
use strum_macros::AsRefStr;
use thiserror::Error;
use zerosumrs::ai::Ai;
use zerosumrs::default_heuristic::DefaultHeuristic;
use zerosumrs::game::{Game, State};
use zerosumrs::heuristic::Heuristic;
use zerosumrs::tablut::Tablut;
use zerosumrs::*;

#[derive(AsRefStr)]
pub enum Player<H: Heuristic<Tablut>> {
    MinimaxSimple(minimax_simple::MinimaxSimple<Tablut, H>),
    MinimaxFinal(minimax_final::MinimaxFinal<Tablut, H>),
    MinimaxKiller(minimax_killer::MinimaxKiller<Tablut, H>),
    MinimaxKillerB(minimax_killer_b::MinimaxKillerB<Tablut, H>),
    Mcts(monte_carlo_tree_search::MonteCarloTreeSearch<Tablut>),
}

impl Ai<Tablut> for Player<DefaultHeuristic> {
    fn new(t: bool) -> Self {
        Player::MinimaxKillerB(minimax_killer_b::MinimaxKillerB::new(t))
    }

    fn state(&self) -> State {
        match &self {
            Self::MinimaxSimple(player) => player.state(),
            Self::MinimaxKiller(player) => player.state(),
            Self::MinimaxKillerB(player) => player.state(),
            Self::MinimaxFinal(player) => player.state(),
            Self::Mcts(player) => player.state(),
        }
    }

    fn print2game(&self) {
        match &self {
            Self::MinimaxSimple(player) => player.print2game(),
            Self::MinimaxKiller(player) => player.print2game(),
            Self::MinimaxKillerB(player) => player.print2game(),
            Self::MinimaxFinal(player) => player.print2game(),
            Self::Mcts(player) => player.print2game(),
        }
    }

    fn turn(&self) -> bool {
        match &self {
            Self::MinimaxSimple(player) => player.turn(),
            Self::MinimaxKiller(player) => player.turn(),
            Self::MinimaxKillerB(player) => player.turn(),
            Self::MinimaxFinal(player) => player.turn(),
            Self::Mcts(player) => player.turn(),
        }
    }

    fn get_mov(&mut self, tl: std::time::Duration) -> <Tablut as Game>::M {
        match self {
            Self::MinimaxSimple(ref mut player) => player.get_mov(tl),
            Self::MinimaxKiller(ref mut player) => player.get_mov(tl),
            Self::MinimaxKillerB(ref mut player) => player.get_mov(tl),
            Self::MinimaxFinal(ref mut player) => player.get_mov(tl),
            Self::Mcts(ref mut player) => player.get_mov(tl),
        }
    }

    fn mov(&mut self, m: &<Tablut as Game>::M) {
        match self {
            Self::MinimaxSimple(ref mut player) => player.mov(m),
            Self::MinimaxKiller(ref mut player) => player.mov(m),
            Self::MinimaxKillerB(ref mut player) => player.mov(m),
            Self::MinimaxFinal(ref mut player) => player.mov(m),
            Self::Mcts(ref mut player) => player.mov(m),
        }
    }

    fn get_game(&self) -> &Tablut {
        match self {
            Self::MinimaxSimple(player) => player.get_game(),
            Self::MinimaxKiller(player) => player.get_game(),
            Self::MinimaxKillerB(player) => player.get_game(),
            Self::MinimaxFinal(player) => player.get_game(),
            Self::Mcts(player) => player.get_game(),
        }
    }
}

#[derive(Error, Debug)]
#[error("Invalid player")]
pub struct ParsePlayerError(String);

impl FromStr for Player<DefaultHeuristic> {
    type Err = ParsePlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("simple") {
            return Ok(Player::MinimaxSimple(minimax_simple::MinimaxSimple::new(
                true,
            )));
        }
        if s.contains("killer") {
            return Ok(Player::MinimaxKiller(minimax_killer::MinimaxKiller::new(
                true,
            )));
        }
        if s.contains("final") {
            return Ok(Player::MinimaxFinal(minimax_final::MinimaxFinal::new(true)));
        }
        if s.contains("mcts") {
            return Ok(Player::Mcts(
                monte_carlo_tree_search::MonteCarloTreeSearch::new(true),
            ));
        }

        Err(ParsePlayerError(s.to_string()))
    }
}

impl Default for Player<DefaultHeuristic> {
    fn default() -> Self {
        <Self as Ai<Tablut>>::new(true)
    }
}
