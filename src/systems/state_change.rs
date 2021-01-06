use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Stats)]
pub fn state_change(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState,
    #[resource] tick: &TickCount,
) {
    // query the player to see if killed or if slowed (and should skip a turn)
    let mut query = <&Stats>::query().filter(component::<Player>());
    let player_stats = query.iter(ecs).next().unwrap();

    let curr_state = turn_state.clone();
    let mut new_state = match curr_state {
        TurnState::InitializeMap => TurnState::AwaitingInput,
        TurnState::AwaitingInput => return,
        TurnState::ComputerTurn => if tick.act(player_stats.speed) {
            TurnState::AwaitingInput
        } else {
            TurnState::ComputerTurn
        },
        TurnState::GameOver => return,
    };

    if player_stats.vigor.is_zero() {
        new_state = TurnState::GameOver;
    }    

    *turn_state = new_state;
}