use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn select_equipped(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    if let Some(item_key) = key {
        match item_key {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        let new_state = match state {
            TurnState::SelectingEquipped(cmd) => {
                match cmd {
                    VirtualKeyCode::F => handle_fire(ecs, commands, item_key),
                    VirtualKeyCode::R => handle_unequip(ecs, commands, item_key),
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
    item: &VirtualKeyCode,
) -> TurnState {

    TurnState::ComputerTurn
}

fn handle_unequip(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer, 
    key: &VirtualKeyCode
) -> TurnState {
    if let Some((_, entity, _)) = key_to_entity(ecs, key) {
        let player = player_entity(ecs);
        unequip_item(ecs, commands, player, entity);
        return TurnState::ComputerTurn
    }

    TurnState::SelectingEquipped(VirtualKeyCode::E)
}

fn key_to_entity<'a>(ecs: &'a SubWorld, key: &VirtualKeyCode) -> Option<(&'a str, Entity, EquipmentSlot)> {
    let inventory = list_of_equipment(ecs);
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