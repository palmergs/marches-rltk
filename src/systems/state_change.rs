use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Stats)]
pub fn state_change(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState
) {
    let curr_state = turn_state.clone();
    let mut new_state = match curr_state {
        TurnState::InitializeMap => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
        TurnState::ComputerTurn => TurnState::AwaitingInput,
        TurnState::GameOver => return,
    };

    let mut query = <&Stats>::query().filter(component::<Player>());
    let is_killed = query.iter(ecs)
        .for_each(|stats| {
            if stats.vigor.is_zero() {
                new_state = TurnState::GameOver;
            }
        });

    *turn_state = new_state;
}