use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Stats)]
#[read_component(Item)]
#[read_component(Stairs)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
    #[resource] turn: &mut TurnCount,
) {
    if let Some(key) = key {
        let mut query = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, location) = query.iter(ecs)
            .find_map(|(entity, pt)| Some((*entity, *pt))).unwrap();
            
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
            VirtualKeyCode::G =>        handle_pickup(ecs, commands, player, location),

            // activate or open by selecting a nearby door, chest, etc 
            VirtualKeyCode::A | VirtualKeyCode::O =>    TurnState::SelectingTarget(VirtualKeyCode::A, None),

            // fire or throw the currently equipped weapon
            VirtualKeyCode::F =>                        TurnState::SelectingTarget(VirtualKeyCode::F, None),

            // talk to the selected item
            VirtualKeyCode::T =>                        TurnState::SelectingTarget(VirtualKeyCode::T, None),

            // look at a item or actor 
            VirtualKeyCode::L =>                        TurnState::SelectingTarget(VirtualKeyCode::L, None),

            // examine or read a carried item
            VirtualKeyCode::X | VirtualKeyCode::R =>    TurnState::SelectingItem(VirtualKeyCode::R),

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
    player: Entity,
    location: Point,
) -> TurnState {
    let mut query = <(Entity, &Point, &Item)>::query();
    query.iter(ecs)
        .filter(|(_, pt, _)| **pt == location )
        .filter(|(_, _, item)| item.is_carryable() )
        .for_each(|(entity, _, _)| {
            commands.push((WantsToGet{ actor: player, item: *entity }, ));
        });
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