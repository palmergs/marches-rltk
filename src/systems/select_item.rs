use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Equipped)]
#[read_component(Carried)]
#[read_component(Render)]
#[read_component(Point)]
#[read_component(Item)]
pub fn select_item(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    println!("in select item...");
    if let Some(item_key) = key {

        let mut query = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, location) = query.iter(ecs)
            .find_map(|(entity, pt)| Some((*entity, *pt))).unwrap();

        match item_key {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                println!("escaping back to awaiting input...");
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        let new_state = match state {
            TurnState::SelectingItem(cmd) => {
                match cmd {
                    VirtualKeyCode::D => handle_drop(ecs, commands, player, item_key),
                    _ => return
                }
            },
            _ => return
        };

        *state = new_state;
    }
}

fn handle_drop(ecs: &SubWorld, commands: &mut CommandBuffer, actor: Entity, key: &VirtualKeyCode) -> TurnState {
    if let Some((_, entity, _)) = key_to_entity(ecs, key) {
        commands.push((WantsToDrop{ actor, item: entity }, ));
        return TurnState::ComputerTurn
    }

    TurnState::SelectingItem(VirtualKeyCode::D)
}

fn key_to_entity<'a>(ecs: &'a SubWorld, key: &VirtualKeyCode) -> Option<(&'a str, Entity, usize)> {
    let inventory = list_of_items(ecs);
    let idx = match key {
        VirtualKeyCode::A => 0,
        VirtualKeyCode::B => 1,
        VirtualKeyCode::C => 2,
        VirtualKeyCode::D => 3,
        VirtualKeyCode::E => 4,
        VirtualKeyCode::F => 5,
        VirtualKeyCode::G => 6,
        VirtualKeyCode::H => 7,
        VirtualKeyCode::I => 8,
        VirtualKeyCode::J => 9,
        VirtualKeyCode::K => 10,
        VirtualKeyCode::L => 11,
        VirtualKeyCode::M => 12,
        VirtualKeyCode::N => 13,
        VirtualKeyCode::O => 14,
        VirtualKeyCode::P => 15,
        VirtualKeyCode::Q => 16,
        VirtualKeyCode::R => 17,
        VirtualKeyCode::S => 18,
        VirtualKeyCode::T => 19,
        VirtualKeyCode::U => 20,
        VirtualKeyCode::V => 21,
        VirtualKeyCode::W => 22,
        VirtualKeyCode::X => 23,
        VirtualKeyCode::Y => 24,
        VirtualKeyCode::Z => 25,
        _ => 9999,
    };
    if idx >= inventory.len() { 
        None
    } else {
        Some(inventory[idx])
    }
}