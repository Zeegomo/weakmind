use crate::game::*;
use crate::heuristic::Heuristic;
use crate::tablut;

pub struct DefaultHeuristic;

impl Heuristic<tablut::Tablut> for DefaultHeuristic {
	fn eval(g: &tablut::Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX - g.turn as i64,
			State::Lose => i64::MIN + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				let nd = g.d.count_ones() as i64;
				let na = g.a.count_ones() as i64;
				let mut km = 0i64;
				let kp = g.k.trailing_zeros();
				let capturer = g.a | tablut::CAPTURE_AID;
				let pass = !(g.a | g.d | tablut::BLOCK);

				let mut i = kp;
				while (pass >> i) & 1 != 0 {
					i += 1;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				i = kp + 11;
				while (pass >> i) & 1 != 0 {
					i += 11;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				i = kp - 1;
				while (pass >> i) & 1 != 0 {
					i -= 1;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				i = kp - 11;
				while (pass >> i) & 1 != 0 {
					i -= 11;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				let ksides = (1u128 << (kp + 1))
					| (1u128 << (kp + 11))
					| (1u128 << (kp - 1))
					| (1u128 << (kp - 11));
				let en_near_k = (capturer & ksides).count_ones();

				nd * 16 + km * 4 - na * 32 - en_near_k as i64 * 10 - (g.turn & 1) as i64
			}
		}
	}
}
