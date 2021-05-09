use crate::ai::Ai;
use crate::game::*;
use crate::heuristic::Heuristic;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

pub struct MinimaxHard<G: Game, H: Heuristic<G>> {
	pub g: G,
	table: HashMap<G::S, (i64, u32)>,
	_ph: PhantomData<H>,
}

impl<G: Game, H: Heuristic<G>> MinimaxHard<G, H> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return H::eval(&self.g);
		}
		let mut old_depth = 0;
		if let Some(x) = self.table.get(&self.g.get_static_state()) {
			if depth <= x.1 {
				return x.0;
			}
			old_depth = x.1;
		}
		let mut res = if self.g.turn() { a } else { b };
		let mut moves = self.g.get_moves();
		moves.sort_by_cached_key(|m| {
			let rb = self.g.mov_with_rollback(m);
			let ans = self
				.table
				.get(&self.g.get_static_state())
				.unwrap_or(&(res, 0))
				.0;
			self.g.rollback(rb);
			if self.g.turn() {
				-ans
			} else {
				ans
			}
		});
		for m in moves.iter() {
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				res = res.max(h);
				a = a.max(h);
			} else {
				res = res.min(h);
				b = b.min(h);
			}
			if a >= b {
				break;
			}
		}
		if depth > old_depth {
			self.table.insert(self.g.get_static_state(), (res, depth));
		}
		res
	}
	fn minimax_move(&mut self, depth: u32) -> G::M {
		if self.g.state() != State::Going || depth == 0 {
			panic!();
		}
		let mut a = i64::MIN;
		let mut b = i64::MAX;
		let mut old_depth = 0;
		if let Some(x) = self.table.get(&self.g.get_static_state()) {
			old_depth = x.1;
		}
		let mut res = if self.g.turn() { a } else { b };

		let mut moves = self.g.get_moves();
		let mut ans = moves[0];
		moves.sort_by_cached_key(|m| {
			let rb = self.g.mov_with_rollback(m);
			let ans = self
				.table
				.get(&self.g.get_static_state())
				.unwrap_or(&(res, 0))
				.0;
			self.g.rollback(rb);
			if self.g.turn() {
				-ans
			} else {
				ans
			}
		});
		for m in moves.iter() {
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				if h > res {
					res = h;
					ans = *m;
				}
				a = a.max(h);
			} else {
				if h < res {
					res = h;
					ans = *m;
				}
				b = b.min(h);
			}
			if a >= b {
				break;
			}
		}
		if depth > old_depth
		/*&& depth > 4*/
		{
			self.table.insert(self.g.get_static_state(), (res, depth));
		}
		ans
	}
}

impl<G: Game, H: Heuristic<G>> Ai<G> for MinimaxHard<G, H> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			table: HashMap::new(),
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
		eprintln!("{}", self.g);
	}
	fn turn(&self) -> bool {
		self.g.turn()
	}
	fn get_mov(&mut self, tl: Duration) -> G::M {
		let start_time = Instant::now();
		let mut depth = 1;
		let mut ans = self.minimax_move(1);
		loop {
			if start_time.elapsed() * 20 > tl {
				break;
			}
			depth += 1;
			ans = self.minimax_move(depth);
		}
		ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
