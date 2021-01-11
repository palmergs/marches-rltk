use crate::prelude::*;

#[system]
pub fn(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,    
) {
    if let Some(item_key) = key {

        let player = <Entity>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .next()
            .unwrap();

        match item_key {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                println!("escaping back to awaiting input from select target...");
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        // let new_state = match state {
        //     TurnState::SelectingItem(cmd) => {
        //         match cmd {
        //             VirtualKeyCode::D => handle_drop(ecs, commands, *player, item_key),
        //             _ => return
        //         }
        //     },
        //     _ => return
        // };

        // *state = new_state;
    }
}