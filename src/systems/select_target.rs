use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn select_target(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,    
) {
    if let Some(dir_key) = key {
        
        println!("key press...");
        match dir_key {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                println!("escaping back to awaiting input from select target...");
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        let player = player_entity(ecs);
        let new_state = match state {
            TurnState::SelectingItem(cmd) => {
                match cmd {
                    VirtualKeyCode::A | VirtualKeyCode::O => handle_activate(ecs, commands, player, *dir_key),
                    _ => return
                }
            },
            _ => return
        };

        *state = new_state;
    }
}

fn handle_activate(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer, 
    player: Entity, 
    dir_key: VirtualKeyCode) -> TurnState {

    TurnState::ComputerTurn
}