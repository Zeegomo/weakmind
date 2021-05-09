use crate::game::Game;

pub trait Heuristic<G: Game> {
	fn eval(g: &G) -> i64;
}
