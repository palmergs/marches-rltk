use crate::prelude::*;

#[system]
pub fn state_change(
    #[resource] turn_state: &mut TurnState
) {
    let curr_state = turn_state.clone();
    let new_state = match curr_state {
        TurnState::InitializeMap => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
        TurnState::ComputerTurn => TurnState::AwaitingInput,
    };
    *turn_state = new_state;
}