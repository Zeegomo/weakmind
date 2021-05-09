use crate::ai::Ai;
use crate::game::*;
use crate::heuristic::Heuristic;
use std::marker::PhantomData;
use std::time::Duration;

pub struct MinimaxFixed<G: Game, H: Heuristic<G>, const D: u32> {
	pub g: G,
	_ph: PhantomData<H>,
}

impl<G: Game, H: Heuristic<G>, const D: u32> MinimaxFixed<G, H, D> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return H::eval(&self.g);
		}
		let moves = self.g.get_moves_sorted();
		for m in moves.iter() {
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				a = a.max(h);
			} else {
				b = b.min(h);
			}
			if a >= b {
				break;
			}
		}
		if self.g.turn() {
			a
		} else {
			b
		}
	}
	fn minimax_move(&mut self, depth: u32) -> G::M {
		let mut a = i64::MIN;
		let mut b = i64::MAX;
		let moves = self.g.get_moves_sorted();
		let mut ans = moves[0];
		for m in moves.iter() {
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				if h > a {
					a = h;
					ans = *m;
				}
			} else {
				if h < b {
					b = h;
					ans = *m;
				}
			}
			if a >= b {
				break;
			}
		}
		ans
	}
}

impl<G: Game, H: Heuristic<G>, const D: u32> Ai<G> for MinimaxFixed<G, H, D> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			_ph: PhantomData,
		}
	}
	fn state(&self) -> State {
		self.g.state()
	}

	fn get_game(&self) -> &G {
		&self.g
	}
	fn print2game(&self) {
		eprintln!("{}", self.g)
	}
	fn turn(&self) -> bool {
		self.g.turn()
	}
	fn get_mov(&mut self, _tl: Duration) -> G::M {
		eprintln!("minimax_fixed depth {}", D);
		self.minimax_move(D)
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
