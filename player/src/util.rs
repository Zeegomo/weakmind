use super::Error;
use crate::common::State;
use zerosumrs::game::Game;
use zerosumrs::tablut::Tablut;

pub fn mov_from_state(
    mut cur_game: Tablut,
    next_state: &State,
) -> Result<<Tablut as Game>::M, Error> {
    for mov in cur_game.get_moves() {
        let rb = cur_game.mov_with_rollback(&mov);
        if &State::from(&cur_game) == next_state {
            return Ok(mov);
        }
        cur_game.rollback(rb);
    }

    let state: State = next_state.clone();
    Err(Error::InvalidNextState(Box::new(state)))
}
