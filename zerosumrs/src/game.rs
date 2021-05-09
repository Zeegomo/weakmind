use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum State {
	Win,
	Lose,
	Draw,
	Going,
}
pub trait Game: Clone + Debug + Display {
	type M: Copy + Eq + Debug + Default;
	type S: Hash + Copy + Eq + Debug;
	type R: Copy + Debug + Default;
	fn new(t: bool) -> Self;
	fn turn(&self) -> bool;
	fn get_moves(&self) -> Vec<Self::M>;
	fn get_moves_sorted(&self) -> Vec<Self::M>;
	fn get_static_state(&self) -> Self::S;
	fn state(&self) -> State;
	fn mov(&mut self, m: &Self::M);
	fn mov_with_rollback(&mut self, m: &Self::M) -> Self::R;
	fn rollback(&mut self, rb: Self::R);
}
