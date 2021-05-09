use crate::ai::Ai;
use crate::game::*;
use crate::heuristic::Heuristic;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

pub struct MinimaxSimple<G: Game, H: Heuristic<G>> {
	pub g: G,
	nnw: u8,
	tl: Duration,
	st: Instant,
	last_ans: G::M,
	ended_early: bool,
	_ph: PhantomData<H>,
}

impl<G: Game, H: Heuristic<G>> MinimaxSimple<G, H> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return H::eval(&self.g);
		}
		self.nnw = self.nnw.wrapping_add(1);
		if self.ended_early || (self.nnw == 0 && self.st.elapsed() > self.tl) {
			self.ended_early = true;
			return if self.g.turn() { a } else { b };
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
			if a >= b || self.ended_early {
				break;
			}
		}
		if self.g.turn() {
			a
		} else {
			b
		}
	}
	fn minimax_move(&mut self, depth: u32) -> bool {
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
			if a >= b || self.ended_early {
				break;
			}
		}
		if self.ended_early {
			true
		} else {
			self.last_ans = ans;
			false
		}
	}
}

impl<G: Game, H: Heuristic<G>> Ai<G> for MinimaxSimple<G, H> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			nnw: 0,
			tl: Duration::ZERO,
			st: Instant::now(),
			last_ans: G::M::default(),
			ended_early: false,
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
	fn get_mov(&mut self, tl: Duration) -> G::M {
		let mut depth = 1;
		self.tl = tl - Duration::from_millis(20);
		self.st = Instant::now();
		self.ended_early = false;
		while !self.minimax_move(depth) {
			depth += 1;
		}
		eprintln!("minimax_simple depth {}", depth - 1);
		self.last_ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
