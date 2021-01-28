use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Render)]
#[read_component(Point)]
#[read_component(Actor)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
#[read_component(Item)]
#[read_component(Equipped)]
#[read_component(Carried)]
#[read_component(Stats)]
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
                *state = TurnState::AwaitingInput;
                return
            },
            _ => ()
        }
        
        if let Some(pt) = point_in_direction(*dir_key) {
            let new_state = match state {
                TurnState::SelectingTarget(cmd) => {
                    match cmd {
                        VirtualKeyCode::A | VirtualKeyCode::O => {
                            let pt = pt + player_pt;
                            if let Some(item) = item_at(ecs, pt, true) {
                                handle_activate(ecs, commands, map, player, item, pt)
                            } else {
                                TurnState::AwaitingInput
                            }
                        },

                        VirtualKeyCode::F => {
                            if let Some((actor, actor_pt)) = actor_first_at(ecs, map, player_pt, pt, 10) {
                                handle_fire(ecs, commands, map, player, actor, actor_pt)
                            } else {
                                TurnState::ComputerTurn
                            }
                        }
                        _ => return
                    }
                },
                _ => return
            };
            *state = new_state;
        };

    } else if click.0 {
        let map_point = camera.offset() + *point;
        let distance = DistanceAlg::Pythagoras.distance2d(player_pt, map_point);
        let new_state = match state {
            TurnState::SelectingTarget(cmd) => {
                match cmd {
                    VirtualKeyCode::A | VirtualKeyCode::O => {
                        if distance < 2.0 {
                            if let Some(item) = item_at(ecs, map_point, true) {
                                handle_activate(ecs, commands, map, player, item, map_point)                 
                            } else {
                                TurnState::SelectingTarget(*cmd)
                            }
                        } else {
                            TurnState::SelectingTarget(*cmd)
                        }
                    },
                    _ => TurnState::ComputerTurn,
                }
            },  
            _ => TurnState::ComputerTurn
        };

        click.0  = false;
        *state = new_state;
    }
}



fn handle_activate(
    ecs: &SubWorld, 
    commands: &mut CommandBuffer, 
    map: &mut Map,
    player: Entity, 
    item: Entity, 
    item_pt: Point,
) -> TurnState {

    if let Ok(item_comp) = ecs.entry_ref(item).unwrap().get_component::<Item>() {
        match &item_comp.id[..] {
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

fn handle_fire(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    _map: &mut Map,
    player: Entity,
    actor: Entity,
    actor_pt: Point,
) -> TurnState {
    if let Some(item) = player_weapon(ecs) {
        drop_item_at(ecs, commands, item, actor_pt);       
        commands.push((WantsToAttack{ actor: player, victim: actor }, ));
    } else {
        commands.push((Text{
            display: TextDisplay::AnimateUp(actor_pt),
            text: format!("Nothing to fire...").to_string(),
            color: RGBA::from_f32(1., 1., 1., 1.),
            ticks: 200,
            count: 0,
        },));
    }

    TurnState::ComputerTurn
}

// Convert a key code to a vector as Point
fn point_in_direction(dir: VirtualKeyCode) -> Option<Point> {
    match dir {
        VirtualKeyCode::Left  | VirtualKeyCode::Numpad4 | VirtualKeyCode::Key4 => Some(Point::new(-1, 0)),
        VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::Key6 => Some(Point::new(1, 0)),
        VirtualKeyCode::Up    | VirtualKeyCode::Numpad8 | VirtualKeyCode::Key8 => Some(Point::new(0, -1)),
        VirtualKeyCode::Down  | VirtualKeyCode::Numpad2 | VirtualKeyCode::Key2 => Some(Point::new(0, 1)),
        VirtualKeyCode::Numpad7 | VirtualKeyCode::Key7 => Some(Point::new(-1, -1)),
        VirtualKeyCode::Numpad9 | VirtualKeyCode::Key9 => Some(Point::new(1, -1)),
        VirtualKeyCode::Numpad1 | VirtualKeyCode::Key1 => Some(Point::new(-1, 1)),
        VirtualKeyCode::Numpad3 | VirtualKeyCode::Key3 => Some(Point::new(1, 1)),
        _ => None,
    }
}

// Find the first Actor entity from start in the direction up to max steps away. 
// Returns None if the max range is reached or if a wall is encountered.
fn actor_first_at(
    ecs: &SubWorld, 
    map: &Map, 
    start: Point, 
    dir: Point, 
    max: usize
) -> Option<(Entity, Point)> {
    let mut steps = 0;
    let mut curr = start + dir;
    while steps < max {
        let idx = map.point2d_to_index(curr);
        if map.is_wall(idx) {
           return None;
        }

        if let Some(entity) = actor_at(ecs, curr) {
            return Some((entity, curr));
        }

        curr = curr + dir;
        steps += 1;
    }
    None
}

fn actor_at(ecs: &SubWorld, at: Point) -> Option<Entity> {
    let mut query = <(Entity, &Point)>::query().filter(component::<Actor>());
    match query.iter(ecs).filter(|(_, pt)| **pt == at).next() {
        Some((entity, _)) => Some(*entity),
        None => None,
    }
}

fn item_first_at(
    ecs: &SubWorld, 
    map: &Map, 
    start: Point, 
    dir: Point, 
    max: usize, 
    non_blocking: bool
) -> Option<(Entity, Point)> {
    let mut steps = 0;
    let mut curr = start + dir;
    while steps < max {
        let idx = map.point2d_to_index(curr);
        if map.is_wall(idx) {
            return None;
        }

        if let Some(entity) = item_at(ecs, curr, non_blocking) {
            return Some((entity, curr));
        }

        curr = curr + dir;
        steps += 1;
    }
    None
}

fn item_at(ecs: &SubWorld, at: Point, non_blocking: bool) -> Option<Entity> {
    let mut query = <(Entity, &Point, &Item)>::query();
    match query
        .iter(ecs)
        .filter(|(_, pt, _)| **pt == at)
        .filter(|(_, _, item)| item.blocking || non_blocking)
        .next() {
            
        Some((entity, _, _)) => Some(*entity),
        None => None,
    }
}

fn thing_at(ecs: &SubWorld, at: Point) -> Option<Entity> {
    let mut query = <(Entity, &Point)>::query();
    match query.iter(ecs).filter(|(_, pt)| **pt == at).next() {
        Some((entity, _)) => Some(*entity),
        None => None
    }
}
