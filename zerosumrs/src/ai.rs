use crate::game::*;

pub trait Ai<G: Game> {
	fn new(t: bool) -> Self;
	fn state(&self) -> State;
	fn print2game(&self);
	fn turn(&self) -> bool;
	fn get_mov(&mut self, tl: std::time::Duration) -> G::M;
	fn mov(&mut self, m: &G::M);
	fn get_game(&self) -> &G;
}
