use crate::ai::Ai;
use crate::game::*;
use crate::heuristic::Heuristic;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

pub struct MinimaxKiller<G: Game, H: Heuristic<G>> {
	pub g: G,
	nnw: u8,
	tl: Duration,
	st: Instant,
	cache: HashMap<G::S, i64>,
	best_mov: VecDeque<G::M>,
	ended_early: bool,
	cur_depth: u32,
	_ph: PhantomData<H>,
}

impl<G: Game, H: Heuristic<G>> MinimaxKiller<G, H> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return H::eval(&self.g);
		}
		self.nnw = self.nnw.wrapping_add(1);
		if self.ended_early || (self.nnw == 0 && self.st.elapsed() > self.tl) {
			self.ended_early = true;
			return if self.g.turn() { a } else { b };
		}
		let moves = self.g.get_moves();
		let bm = self.best_mov[depth as usize];
		if moves.contains(&bm) {
			let rb = self.g.mov_with_rollback(&bm);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				a = a.max(h);
			} else {
				b = b.min(h);
			}
			if self.ended_early {
				return if self.g.turn() { a } else { b };
			}
		}
		for m in moves.iter() {
			if *m == bm {
				continue;
			}
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				if h > a {
					a = h;
					if !self.ended_early {
						self.best_mov[depth as usize] = *m;
					}
				}
			} else if h < b {
				b = h;
				if !self.ended_early {
					self.best_mov[depth as usize] = *m;
				}
			}
			if a >= b || self.ended_early {
				break;
			}
		}
		let res = if self.g.turn() { a } else { b };
		res
	}
}

impl<G: Game, H: Heuristic<G>> Ai<G> for MinimaxKiller<G, H> {
	fn new(t: bool) -> Self {
		let mut vd = VecDeque::with_capacity(8);
		vd.push_back(G::M::default());
		Self {
			g: G::new(t),
			nnw: 0,
			tl: Duration::ZERO,
			cache: HashMap::new(),
			st: Instant::now(),
			best_mov: vd,
			ended_early: false,
			cur_depth: 0,
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
		self.tl = tl - Duration::from_millis(20);
		self.st = Instant::now();
		self.ended_early = false;
		while !self.ended_early {
			self.cur_depth += 1;
			self.best_mov.push_front(self.best_mov[0]);
			self.minimax(i64::MIN, i64::MAX, self.cur_depth);
		}
		self.cur_depth -= 1;
		self.best_mov.pop_front();
		eprintln!("minimax_killer depth {}", self.cur_depth);
		*self.best_mov.back().unwrap()
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
		if self.cur_depth != 0 {
			self.cur_depth -= 1;
			self.best_mov.pop_back();
		}
	}
}
