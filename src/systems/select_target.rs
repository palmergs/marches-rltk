use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Render)]
#[read_component(Point)]
#[read_component(Actor)]
#[read_component(Item)]
pub fn select_target(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] click: &mut MouseClick,
    #[resource] point: &Point,
    #[resource] state: &mut TurnState,   
    #[resource] camera: &Camera,
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
    } else if click.0 {
        let map_point = camera.offset() + *point;
        <(Entity, &Render, &Point)>::query()
            .iter(ecs)
            .filter(|(_, _, pt)| **pt == map_point)
            .for_each(|(entity, render, pt)| {

                if let Ok(_) = ecs.entry_ref(*entity).unwrap().get_component::<Actor>() {
                    println!("Found a monster={:?}", render);
                }
    
                if let Ok(_) = ecs.entry_ref(*entity).unwrap().get_component::<Item>() {
                    println!("Found an item={:?}", render);
                }
            });

        click.0 = false;
        *state = TurnState::ComputerTurn;
    }
}

fn handle_activate(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer, 
    player: Entity, 
    dir_key: VirtualKeyCode) -> TurnState {

    TurnState::ComputerTurn
}