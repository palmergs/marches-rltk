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
    if let Some(key) = key {
        println!("key pressed {:?}", key);
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let (player, location) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos))).unwrap();

        match key {
            VirtualKeyCode::Left  | VirtualKeyCode::Numpad4 =>  handle_move(ecs, commands, player, location, Point::new(-1, 0)),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 =>  handle_move(ecs, commands, player, location, Point::new(1, 0)),
            VirtualKeyCode::Up    | VirtualKeyCode::Numpad8 =>  handle_move(ecs, commands, player, location, Point::new(0, -1)),
            VirtualKeyCode::Down  | VirtualKeyCode::Numpad2 =>  handle_move(ecs, commands, player, location, Point::new(0, 1)),
            VirtualKeyCode::Numpad7 =>  handle_move(ecs, commands, player, location, Point::new(-1, -1)),
            VirtualKeyCode::Numpad9 =>  handle_move(ecs, commands, player, location, Point::new(1, -1)),
            VirtualKeyCode::Numpad1 =>  handle_move(ecs, commands, player, location, Point::new(-1, 1)),
            VirtualKeyCode::Numpad3 =>  handle_move(ecs, commands, player, location, Point::new(1, 1)),
            _ => (),
        };
        *state = TurnState::PlayerTurn;
    }
}

fn handle_move(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    actor: Entity,
    from: Point,
    delta: Point
) {
    let destination = from + delta;
    let mut hit_something = false;
    let mut npcs = <(Entity, &Point)>::query().filter(component::<Actor>());
    npcs.iter(ecs)
        .filter(|(_, pos)| { **pos == destination })
        .for_each(|(entity, _)| {
            if *entity != actor {
                hit_something = true;
                commands.push(((), WantsToInteract{ actor, victim: *entity }));
            }
        });

    if !hit_something {
        println!("Adding move command: {:?} to {:?}", actor, destination);
        commands.push(((), WantsToMove{ actor, destination }));
    }
}