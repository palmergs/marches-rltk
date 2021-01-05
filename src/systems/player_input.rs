use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Player)]
#[read_component(Stats)]
#[read_component(Actor)]
#[read_component(FieldOfView)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] input: &PlayerInput,
    #[resource] state: &mut TurnState,
    #[resource] turn: &mut TurnCount,
    #[resource] tick: &TickCount,
    #[resource] camera: &Camera,
    #[resource] map: &Map,
) {
    println!("player input: {:?}", input);
    if let Some(key) = input.key {
        println!("key = {:?}", key);
        let mut players = <(Entity, &Render)>::query().filter(component::<Player>());
        let (player, location) = players
            .iter(ecs)
            .find_map(|(entity, render)| Some((*entity, render.pt))).unwrap();

        match key {
            VirtualKeyCode::Left => if input.shift {
                handle_move(ecs, commands, player, location + Point::new(-1, 1))
            } else {
                handle_move(ecs, commands, player, location + Point::new(-1, 0))
            },
            VirtualKeyCode::Right => if input.shift {
                handle_move(ecs, commands, player, location + Point::new(1, -1))
            } else {
                handle_move(ecs, commands, player, location + Point::new(1, 0))
            },
            VirtualKeyCode::Up => if input.shift {
                handle_move(ecs, commands, player, location + Point::new(-1, -1))
            } else {
                handle_move(ecs, commands, player, location + Point::new(0, -1))
            },
            VirtualKeyCode::Down => if input.shift {
                handle_move(ecs, commands, player, location + Point::new(1, 1))
            } else {
                handle_move(ecs, commands, player, location + Point::new(0, 1))
            },
            VirtualKeyCode::Numpad6 => handle_move(ecs, commands, player, location + Point::new( 1,  0)),
            VirtualKeyCode::Numpad8 => handle_move(ecs, commands, player, location + Point::new( 0, -1)),
            VirtualKeyCode::Numpad4 => handle_move(ecs, commands, player, location + Point::new(-1,  0)),
            VirtualKeyCode::Numpad2 => handle_move(ecs, commands, player, location + Point::new( 0,  1)),
            VirtualKeyCode::Numpad7 => handle_move(ecs, commands, player, location + Point::new(-1, -1)),
            VirtualKeyCode::Numpad9 => handle_move(ecs, commands, player, location + Point::new( 1, -1)),
            VirtualKeyCode::Numpad1 => handle_move(ecs, commands, player, location + Point::new(-1,  1)),
            VirtualKeyCode::Numpad3 => handle_move(ecs, commands, player, location + Point::new( 1,  1)),
            _ => (),
        };

        turn.increment();
        *state = TurnState::ComputerTurn;
    } else if input.click {
        let pt = input.mouse + camera.offset();
        let mut query = <(Entity, &Render, &FieldOfView)>::query().filter(component::<Player>());
        let (player, render, fov) = query.iter(ecs).next().unwrap();
        if fov.visible_tiles.contains(&pt) {
            let path = a_star_search(map.point2d_to_index(pt), map.point2d_to_index(input.mouse), map);
            if path.success {
                let mut it = path.steps.iter().skip(1);
                if let Some(idx) = it.next() {
                    let pt = map.index_to_point2d(*idx);
                    println!("mouse move to {:?}", pt);
                    handle_move(ecs, commands, *player, pt);
                    turn.increment();
                    *state = TurnState::ComputerTurn;
                }
            }
        }
    }
}

fn handle_move(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    actor: Entity,
    destination: Point,
) {
    let mut hit_something = false;
    let mut npcs = <(Entity, &Render)>::query().filter(component::<Actor>());
    npcs.iter(ecs)
        .filter(|(_, render)| { render.pt == destination })
        .for_each(|(entity, _)| {
            if *entity != actor {
                hit_something = true;
                commands.push(((), WantsToAttack{ actor, victim: *entity }));
            }
        });

    if !hit_something {
        commands.push(((), WantsToMove{ actor, destination }));
    }
}
