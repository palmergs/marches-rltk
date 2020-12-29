use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Actor)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] state: &mut TurnState,
) {
    if state.clone() != TurnState::AwaitingInput { return; }

    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, location) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos))).unwrap();

        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => return handle_move(ecs, commands, player, location, Point::new(-1, 0)),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => return handle_move(ecs, commands, player, location, Point::new(1, 0)),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => return handle_move(ecs, commands, player, location, Point::new(0, -1)),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => return handle_move(ecs, commands, player, location, Point::new(0, 1)),
            VirtualKeyCode::Numpad7 => return handle_move(ecs, commands, player, location, Point::new(-1, -1)),
            VirtualKeyCode::Numpad9 => return handle_move(ecs, commands, player, location, Point::new(1, -1)),
            VirtualKeyCode::Numpad1 => return handle_move(ecs, commands, player, location, Point::new(-1, 1)),
            VirtualKeyCode::Numpad3 => return handle_move(ecs, commands, player, location, Point::new(1, 1)),
            _ => Point::zero(),
        };
    }
}

fn handle_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    player: Entity,
    from: Point,
    delta: Point
) {
    let destination = from + delta;
    let mut hit_something = false;
    let mut npcs = <(Entity, &Point)>::query().filter(component::<Actor>());
    npcs.iter(ecs)
        .filter(|(_, pos)| { **pos == destination })
        .for_each(|(entity, _)| {
            if *entity != player {
                hit_something = true;
                commands.push(((), WantsToInteract{ actor: player, victim: *entity }));
            }
        });

    if !hit_something {
        commands.push(((), WantsToMove{ entity: player, destination }));
    }
}