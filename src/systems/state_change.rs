use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Stats)]
pub fn state_change(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] tick: &TickCount,
) {
    let player_stats = player_stats(ecs);

    let curr_state = turn_state.clone();
    let mut new_state = match curr_state {
        TurnState::InitializeMap => TurnState::AwaitingInput,
        TurnState::ComputerTurn => if tick.act(player_stats.speed) {
            TurnState::AwaitingInput
        } else {
            TurnState::ComputerTurn
        },
        _ => return,
    };

    if player_stats.vigor.is_zero() {
        new_state = TurnState::GameOver;
    }    

    *turn_state = new_state;
}