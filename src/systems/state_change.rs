use crate::prelude::*;

#[system]
pub fn state_change(
    #[resource] turn_state: &mut TurnState
) {
    let curr_state = turn_state.clone();
    let new_state = match curr_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::ComputerTurn,
        TurnState::ComputerTurn => TurnState::AwaitingInput,
        _ => curr_state,
    };
    *turn_state = new_state;
}