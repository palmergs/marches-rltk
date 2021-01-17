use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Render)]
#[read_component(Point)]
#[read_component(Actor)]
#[read_component(Item)]
pub fn select_target(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] click: &mut MouseClick,
    #[resource] point: &Point,
    #[resource] map: &mut Map,
    #[resource] state: &mut TurnState,   
    #[resource] camera: &Camera,
) {
    let (player, player_pt) = player_at(ecs);
    if let Some(dir_key) = key {
        match dir_key {
            VirtualKeyCode::Escape | VirtualKeyCode::Back | VirtualKeyCode::Delete => {
                println!("escaping back to awaiting input from select target...");
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }

        let new_state = match state {
            TurnState::SelectingItem(cmd) => {
                match cmd {
                    VirtualKeyCode::A | VirtualKeyCode::O => {
                        if let Some((item, item_pt)) = item_at_direction(ecs, player_pt, *dir_key) {
                            handle_activate(ecs, commands, map, player, item, item_pt)
                        } else {
                            return
                        }
                    },
                    _ => return
                }
            },
            _ => return
        };

        *state = new_state;
    } else if click.0 {
        let map_point = camera.offset() + *point;
        let distance = DistanceAlg::Pythagoras.distance2d(player_pt, map_point);
        let mut new_state = *state;
        <(Entity, &Render, &Point)>::query()
            .iter(ecs)
            .filter(|(_, _, pt)| **pt == map_point)
            .for_each(|(entity, render, pt)| {

                if let Ok(_) = ecs.entry_ref(*entity).unwrap().get_component::<Actor>() {
                    println!("Found a monster={:?}", render);
                }
    
                if let Ok(_) = ecs.entry_ref(*entity).unwrap().get_component::<Item>() {
                    new_state = match state {
                        TurnState::SelectingItem(cmd) => {
                            match cmd {
                                VirtualKeyCode::A | VirtualKeyCode::O => if distance < 2.0 {
                                    handle_activate(ecs, commands, map, player, *entity, *pt)
                                } else {
                                    return
                                },
                                _ => return
                            }
                        },
                        _ => return
                    };
                }
            });

        click.0 = false;
        *state = TurnState::ComputerTurn;
    }
}

fn item_at_direction(ecs: &SubWorld, player_pt: Point, dir: VirtualKeyCode) -> Option<(Entity, Point)> {
    let item_pt = match dir {
        VirtualKeyCode::Left  | VirtualKeyCode::Numpad4 | VirtualKeyCode::Key4 => Point::new(-1, 0),
        VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::Key6 => Point::new(1, 0),
        VirtualKeyCode::Up    | VirtualKeyCode::Numpad8 | VirtualKeyCode::Key8 => Point::new(0, -1),
        VirtualKeyCode::Down  | VirtualKeyCode::Numpad2 | VirtualKeyCode::Key2 => Point::new(0, 1),
        VirtualKeyCode::Numpad7 | VirtualKeyCode::Key7 => Point::new(-1, -1),
        VirtualKeyCode::Numpad9 | VirtualKeyCode::Key9 => Point::new(1, -1),
        VirtualKeyCode::Numpad1 | VirtualKeyCode::Key1 => Point::new(-1, 1),
        VirtualKeyCode::Numpad3 | VirtualKeyCode::Key3 => Point::new(1, 1),
        _ => return None,
    };

    let item_pt = player_pt + item_pt;
    if let Some((entity, _)) = <(Entity, &Point)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, pt)| **pt == item_pt)
        .next() {

        return Some((*entity, item_pt));
    }

    return None
}

fn handle_activate(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer, 
    map: &mut Map,
    player: Entity, 
    item: Entity, 
    item_pt: Point,) -> TurnState {

    if let Ok(render) = ecs.entry_ref(item).unwrap().get_component::<Render>() {
        match &render.name[..] {
            "open door" => {
                commands.remove(item);
                commands.push(closed_door_tuple(item_pt));
                map.opaque.insert(item_pt);
                map.blocked.insert(item_pt);
                player_view_dirty(ecs, commands, player);
            },
            "closed door" => {
                commands.remove(item);
                commands.push(open_door_tuple(item_pt));
                map.opaque.remove(&item_pt);
                map.blocked.remove(&item_pt);
                player_view_dirty(ecs, commands, player);
            }
            _ => ()
        }
    }

    TurnState::ComputerTurn
}