use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn select_equipped(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    if let Some(item_key) = key {

        let player = player_entity(ecs);

        match item_key {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        let new_state = match state {
            TurnState::SelectingItem(cmd) => {
                match cmd {
                    VirtualKeyCode::F => handle_fire(ecs, commands, player, *item_key),
                    VirtualKeyCode::R => handle_unequip(ecs, commands, player, *item_key),
                    _ => return
                }
            },
            _ => return
        };

        *state = new_state;
    }
}

fn handle_fire(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer, 
    player: Entity, 
    item: VirtualKeyCode,
) -> TurnState {

    TurnState::ComputerTurn
}

fn handle_unequip(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer, 
    player: Entity, 
    item: VirtualKeyCode,
) -> TurnState {

    TurnState::ComputerTurn
}