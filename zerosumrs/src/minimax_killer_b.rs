use crate::ai::Ai;
use crate::game::*;
use crate::heuristic::Heuristic;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

pub struct MinimaxKillerB<G: Game, H: Heuristic<G>> {
	pub g: G,
	nnw: u8,
	tl: Duration,
	st: Instant,
	best_mov: VecDeque<G::M>,
	global_best: VecDeque<G::M>,
	ended_early: bool,
	cur_depth: u32,
	_ph: PhantomData<H>,
	iterations: u64,
}

impl<G: Game, H: Heuristic<G>> MinimaxKillerB<G, H> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32, best: bool) -> (i64, VecDeque<G::M>) {
		self.iterations += 1;
		let mut mv = VecDeque::with_capacity(self.cur_depth as usize + 1);
		for _ in 0..depth + 1 {
			mv.push_back(G::M::default());
		}
		if self.g.state() != State::Going || depth == 0 {
			return (H::eval(&self.g), mv);
		}
		self.nnw = self.nnw.wrapping_add(1);
		if self.ended_early || (self.nnw == 0 && self.st.elapsed() > self.tl) {
			self.ended_early = true;
			return (if self.g.turn() { a } else { b }, mv);
		}

		let moves = self.g.get_moves();
		let bm = if best {
			self.global_best[depth as usize]
		} else {
			self.best_mov[depth as usize]
		};
		if moves.contains(&bm) {
			let rb = self.g.mov_with_rollback(&bm);
			let (h, hv) = self.minimax(a, b, depth - 1, best);
			self.g.rollback(rb);
			if self.g.turn() {
				if h > a {
					a = h;
					if !self.ended_early {
						self.best_mov[depth as usize] = bm;
						mv = hv;
						mv.push_back(bm);
					}
				}
			} else if h < b {
				b = h;
				if !self.ended_early {
					self.best_mov[depth as usize] = bm;
					mv = hv;
					mv.push_back(bm);
				}
			}
		}

		for m in moves.iter() {
			if *m == bm {
				continue;
			}
			let rb = self.g.mov_with_rollback(m);
			let (h, hv) = self.minimax(a, b, depth - 1, false);
			self.g.rollback(rb);
			if self.g.turn() {
				if h > a {
					a = h;
					if !self.ended_early {
						self.best_mov[depth as usize] = *m;
						mv = hv;
						mv.push_back(*m);
					}
				}
			} else if h < b {
				b = h;
				if !self.ended_early {
					self.best_mov[depth as usize] = *m;
					mv = hv;
					mv.push_back(*m);
				}
			}
			if a >= b || self.ended_early {
				break;
			}
		}
		if self.g.turn() {
			(a, mv)
		} else {
			(b, mv)
		}
	}
}

impl<G: Game, H: Heuristic<G>> Ai<G> for MinimaxKillerB<G, H> {
	fn new(t: bool) -> Self {
		let mut vd = VecDeque::with_capacity(8);
		vd.push_back(G::M::default());
		Self {
			g: G::new(t),
			nnw: 0,
			tl: Duration::ZERO,
			st: Instant::now(),
			best_mov: vd.clone(),
			global_best: vd,
			ended_early: false,
			cur_depth: 0,
			_ph: PhantomData,
			iterations: 0,
		}
	}

	fn get_game(&self) -> &G {
		&self.g
	}

	fn state(&self) -> State {
		self.g.state()
	}
	fn print2game(&self) {
		eprintln!("{}", self.g)
	}
	fn turn(&self) -> bool {
		self.g.turn()
	}
	fn get_mov(&mut self, tl: Duration) -> G::M {
		self.tl = tl - Duration::from_millis(20);
		self.iterations = 0;
		self.st = Instant::now();
		self.ended_early = false;
		let mut val = 0;
		while !self.ended_early {
			self.cur_depth += 1;
			self.best_mov.push_front(self.best_mov[0]);
			self.global_best.push_front(self.global_best[0]);
			let (h, hv) = self.minimax(i64::MIN, i64::MAX, self.cur_depth, true);
			if !self.ended_early {
				self.global_best = hv;
				val = h;
			}
		}
		self.cur_depth -= 1;
		self.best_mov.pop_front();
		self.global_best.pop_front();
		eprintln!(
			"minimax_killer_b depth {} val {} it {}",
			self.cur_depth, val, self.iterations
		);
		*self.best_mov.back().unwrap()
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
		if self.cur_depth != 0 {
			self.cur_depth -= 1;
			self.best_mov.pop_back();
			self.global_best.pop_back();
		}
	}
}
