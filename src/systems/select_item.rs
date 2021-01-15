use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Equipped)]
#[read_component(Equippable)]
#[read_component(Carried)]
#[read_component(Render)]
#[read_component(FieldOfLight)]
#[read_component(FieldOfView)]
#[read_component(Consumable)]
#[read_component(RestoresVigor)]
#[read_component(RestoresFocus)]
#[read_component(Stats)]
#[read_component(Point)]
#[read_component(Item)]
pub fn select_item(
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
            TurnState::SelectingItem(cmd) => {
                match cmd {
                    VirtualKeyCode::D => handle_drop(ecs, commands, item_key),
                    VirtualKeyCode::E | VirtualKeyCode::W => handle_equip(ecs, commands, item_key),
                    VirtualKeyCode::U => handle_use(ecs, commands, item_key),
                    _ => return
                }
            },
            _ => return
        };

        *state = new_state;
    }
}

fn handle_drop(ecs: &mut SubWorld, commands: &mut CommandBuffer, key: &VirtualKeyCode) -> TurnState {
    if let Some((_, entity, _)) = key_to_entity(ecs, key) {
        drop_item(ecs, commands, entity);
        return TurnState::ComputerTurn
    }

    TurnState::SelectingItem(VirtualKeyCode::D)
}

fn handle_equip(ecs: &mut SubWorld, commands: &mut CommandBuffer, key: &VirtualKeyCode) -> TurnState {
    if let Some((_, entity, _)) = key_to_entity(ecs, key) {
        let player = player_entity(ecs);
        equip_item(ecs, commands, player, entity);
        return TurnState::ComputerTurn
    }

    TurnState::SelectingItem(VirtualKeyCode::E)
} 

fn handle_use(ecs: &mut SubWorld, commands: &mut CommandBuffer, key: &VirtualKeyCode) -> TurnState {
    if let Some((_, entity, _)) = key_to_entity(ecs, key) {
        let (player, player_pt) = player_at(ecs);
        let item_ref = ecs.entry_ref(entity).unwrap();
        if let Ok(consumable) = item_ref.get_component::<Consumable>() {
            if let Ok(restores_vigor) = item_ref.get_component::<RestoresVigor>() {
                let amount = restores_vigor.amount;
                if let Ok(player_stats) = ecs.entry_ref(player).unwrap().get_component::<Stats>() {
                    if amount > 0 {
                        let mut stats = player_stats.clone();
                        let amount = stats.vigor.heal(amount);
                        stats.vigor.curr += amount;
                        commands.add_component(player, stats);
                        commands.push((Text{
                            display: TextDisplay::AnimateUp(player_pt),
                            text: format!("{}", amount),
                            color: RGBA::named(PINK),
                            ticks: 50,
                            count: 0,
                        }, ));
                    }

                    if amount < 0 {
                        let mut stats = player_stats.clone();
                        let amount = std::cmp::min(stats.vigor.curr, amount);
                        stats.vigor.curr -= amount;
                        commands.add_component(player, stats);
                        commands.push((Text{
                            display: TextDisplay::AnimateUp(player_pt),
                            text: format!("{}", amount),
                            color: RGBA::named(RED),
                            ticks: 50,
                            count: 0,
                        }, ));
                    }
                }
            }

            if let Ok(restores_focus) = item_ref.get_component::<RestoresFocus>() {
                let amount = restores_focus.amount;
                if let Ok(player_stats) = ecs.entry_ref(player).unwrap().get_component::<Stats>() {
                    if amount > 0 {
                        let mut stats = player_stats.clone();
                        let amount = stats.focus.heal(amount);
                        stats.focus.curr += amount;
                        commands.add_component(player, stats);
                        commands.push((Text{
                            display: TextDisplay::AnimateUp(player_pt),
                            text: format!("{}", amount),
                            color: RGBA::named(CYAN),
                            ticks: 50,
                            count: 0,
                        }, ));
                    }

                    if amount < 0 {
                        let mut stats = player_stats.clone();
                        let amount = std::cmp::min(stats.focus.curr, amount);
                        stats.focus.curr -= amount;
                        commands.add_component(player, stats);
                        commands.push((Text{
                            display: TextDisplay::AnimateUp(player_pt),
                            text: format!("{}", amount),
                            color: RGBA::named(RED),
                            ticks: 50,
                            count: 0,
                        }, ));
                    }
                }
            }

            if consumable.count > 1 {
                commands.add_component(entity, Consumable{ count: consumable.count - 1 });
            } else {
                commands.remove(entity);
            }
        }
    }

    TurnState::ComputerTurn
}

fn key_to_entity<'a>(ecs: &'a SubWorld, key: &VirtualKeyCode) -> Option<(&'a str, Entity, usize)> {
    let inventory = list_of_item_counts(ecs);
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
