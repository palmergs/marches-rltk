use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Stats)]
#[read_component(Carried)]
#[read_component(Equippable)]
#[read_component(Item)]
#[read_component(Stairs)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
    #[resource] turn: &mut TurnCount,
) {
    if let Some(key) = key {
        let (player, location) = player_at(ecs);
            
        let new_state = match key {
            VirtualKeyCode::Left  | VirtualKeyCode::Numpad4 | VirtualKeyCode::Key4 => handle_move(ecs, commands, player, location, Point::new(-1, 0)),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::Key6 => handle_move(ecs, commands, player, location, Point::new(1, 0)),
            VirtualKeyCode::Up    | VirtualKeyCode::Numpad8 | VirtualKeyCode::Key8 => handle_move(ecs, commands, player, location, Point::new(0, -1)),
            VirtualKeyCode::Down  | VirtualKeyCode::Numpad2 | VirtualKeyCode::Key2 => handle_move(ecs, commands, player, location, Point::new(0, 1)),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Key7 => handle_move(ecs, commands, player, location, Point::new(-1, -1)),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Key9 => handle_move(ecs, commands, player, location, Point::new(1, -1)),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::Key1 => handle_move(ecs, commands, player, location, Point::new(-1, 1)),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::Key3 => handle_move(ecs, commands, player, location, Point::new(1, 1)),

            VirtualKeyCode::Period =>   handle_stairs(ecs, location),
            VirtualKeyCode::Comma =>    handle_stairs(ecs, location),
            VirtualKeyCode::G =>        handle_pickup(ecs, commands, location),

            // activate or open by selecting a nearby door, chest, etc 
            VirtualKeyCode::A | VirtualKeyCode::O =>    TurnState::SelectingTarget(VirtualKeyCode::A, None),

            // talk to the selected item
            VirtualKeyCode::T =>                        TurnState::SelectingTarget(VirtualKeyCode::T, None),

            // look at a item or actor 
            VirtualKeyCode::L =>                        TurnState::SelectingTarget(VirtualKeyCode::L, None),

            // fire or throw the currently equipped weapon
            VirtualKeyCode::F =>                        TurnState::SelectingEquipped(VirtualKeyCode::F),

            // remove or unequip item
            VirtualKeyCode::R =>                        TurnState::SelectingEquipped(VirtualKeyCode::R),

            // examine or read a carried item
            VirtualKeyCode::X =>                        TurnState::SelectingItem(VirtualKeyCode::R),

            // drop a carried item (unequipping it if necessary)
            VirtualKeyCode::D =>                        TurnState::SelectingItem(VirtualKeyCode::D),

            // inventory with no secondary action
            VirtualKeyCode::I =>                        TurnState::SelectingItem(VirtualKeyCode::I),            

            // use a carried item
            VirtualKeyCode::U =>                        TurnState::SelectingItem(VirtualKeyCode::U),

            // equip or wield or wear an item
            VirtualKeyCode::W | VirtualKeyCode::E =>    TurnState::SelectingItem(VirtualKeyCode::E),            
            

            _ => TurnState::ComputerTurn,
        };

        turn.increment();
        *state = new_state;
    }
}

fn handle_pickup(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    location: Point,
) -> TurnState {
    let items = <(Entity, &Point, &Item)>::query()
        .iter(ecs)
        .filter(|(_, pt, _)| **pt == location)
        .filter(|(_, _, item)| item.can_get)
        .map(|(entity, _, _)| *entity)
        .collect::<Vec<_>>();
    for item in items.into_iter() {
        get_item(ecs, commands, item);
    }
    TurnState::ComputerTurn
}

fn handle_stairs(
    ecs: &mut SubWorld,
    location: Point,
) -> TurnState {
    let mut query = <(&Point, &Stairs)>::query();
    match query.iter(ecs).filter(|(pt, _)| **pt == location ).next() {
        Some((_, stairs)) => return TurnState::NewLevel(stairs.to_depth),
        None => TurnState::ComputerTurn,
    }
}

fn handle_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    player: Entity,
    from: Point,
    delta: Point
) -> TurnState {
    let destination = from + delta;
    let mut hit_something = false;
    let mut query = <(Entity, &Point)>::query().filter(component::<Stats>());
    query.iter(ecs)
        .filter(|(_, pt)| { **pt == destination })
        .for_each(|(entity, _)| {
            if *entity != player {
                if let Ok(item) = ecs.entry_ref(*entity).unwrap().get_component::<Item>() {
                    if item.blocking {
                        hit_something = true;
                        commands.push((WantsToAttack{ actor: player, victim: *entity }, ));
                    }
                } else {
                    hit_something = true;
                    commands.push((WantsToAttack{ actor: player, victim: *entity }, ));
                }
            }
        });

    if !hit_something {
        commands.push((WantsToMove{ actor: player, destination }, ));
    }

    TurnState::ComputerTurn
}