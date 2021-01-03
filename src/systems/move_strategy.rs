use crate::prelude::*;

use std::collections::HashMap;

#[system]
#[read_component(Player)]
#[read_component(Actor)]
#[read_component(Render)]
#[read_component(MoveStrategy)]
#[read_component(Outlook)]
#[read_component(FieldOfView)]
// #[write_component(WantsToMove)]
pub fn move_strategy(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] tick: &TickCount,
) {
    // points containing actors
    let mut npcs: HashMap<Point, Entity> = HashMap::new();
    <(Entity, &Render)>::query()
        .filter(component::<Actor>())
        .iter(ecs)
        .for_each(|(entity, render)| { npcs.insert(render.pt, *entity); });

    // location and entity for player
    let mut query = <(Entity, &Player, &Render)>::query();
    let (player_entity, _, player_render) = query.iter(ecs).next().unwrap();
    
    let player_pos = player_render.pt;
    let player_idx = map.point2d_to_index(player_pos);
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &search_targets,
        map,
        1024.0);

    let mut rng = Rng::new();
    let mut query = <(Entity, &MoveStrategy, &Render, &Outlook, &FieldOfView)>::query();
    query.iter(ecs).for_each(|(entity, strategy, render, outlook, fov)| {
        match strategy {
            MoveStrategy::Patrol(n, dir) => {
                if tick.act(*n) {
                    if fov.visible_tiles.contains(&player_pos) {
                        match outlook {
                            Outlook::Aggressive => {
                                commands.add_component(*entity, MoveStrategy::Chase(*n));
                                chase(entity, commands, player_entity, &render.pt, map, &npcs);
                            },
                            Outlook::Neutral => {
                                patrol(entity, commands, player_entity, &render.pt, *n, dir, map, &npcs);
                            },
                            Outlook::Fearful => {
                                commands.add_component(*entity, MoveStrategy::Flee(*n));
                                flee(entity, commands, player_entity, &render.pt, map, &npcs);
                            }
                        }
                    } else {
                        patrol(entity, commands, player_entity, &render.pt, *n, dir, map, &npcs);
                    }
                }
            },
            MoveStrategy::Random(n) => {
                if tick.act(*n) {
                    if fov.visible_tiles.contains(&player_render.pt) {
                        match outlook {
                            Outlook::Aggressive => {
                                commands.add_component(*entity, MoveStrategy::Chase(*n));
                                chase(entity, commands, player_entity, &render.pt, map, &npcs);
                            },
                            Outlook::Neutral => {
                                random(entity, commands, player_entity, &render.pt, map, &npcs, &mut rng);
                            },
                            Outlook::Fearful => {
                                commands.add_component(*entity, MoveStrategy::Flee(*n));
                                flee(entity, commands, player_entity, &render.pt, map, &npcs);
                            }
                        }
                    } else {
                        random(entity, commands, player_entity, &render.pt, map, &npcs, &mut rng);
                    }         
                }       
            },
            MoveStrategy::Chase(n) => {
                if tick.act(*n) {
                    match outlook {
                        Outlook::Fearful => {
                            commands.add_component(*entity, MoveStrategy::Flee(*n));
                            flee(entity, commands, player_entity, &render.pt, map, &npcs);
                        },
                        _ => chase(entity, commands, player_entity, &render.pt, map, &npcs)
                    }
                }
            },
            MoveStrategy::Flee(n) => {
                if tick.act(*n) {
                    match outlook {
                        Outlook::Aggressive => {
                            commands.add_component(*entity, MoveStrategy::Chase(*n));
                            chase(entity, commands, player_entity, &render.pt, map, &npcs);
                        },
                        _ => flee(entity, commands, player_entity, &render.pt, map, &npcs)
                    }
                }                
            }
        }
    });
}

fn chase(
    entity: &Entity, 
    commands: &mut CommandBuffer, 
    player_entity: &Entity, 
    pt: &Point, 
    map: &Map,
    npcs: &HashMap<Point, Entity>
) {

}

fn flee(
    entity: &Entity, 
    commands: &mut CommandBuffer, 
    player_entity: &Entity, 
    pt: &Point, 
    map: &Map,
    npcs: &HashMap<Point, Entity>
) {

}

fn random(
    entity: &Entity, 
    commands: &mut CommandBuffer, 
    player_entity: &Entity, 
    pt: &Point, 
    map: &Map, 
    npcs: &HashMap<Point, Entity>,
    rng: &mut Rng
) {
    let destination = match rng.range(0, 12) {
        0 | 1 => Point::new(-1,  0) + *pt,
        2 | 3 => Point::new( 1,  0) + *pt,
        4 | 5 => Point::new( 0, -1) + *pt,
        6 | 7 => Point::new( 0,  1) + *pt,
        8     => Point::new(-1, -1) + *pt,
        9     => Point::new( 1, -1) + *pt,
        10    => Point::new(-1,  1) + *pt,
        11    => Point::new( 1,  1) + *pt,
        _     => *pt,
    };

    if destination != *pt {
        if let Some(npc_entity) = npcs.get(&destination) {
            if npc_entity == player_entity {
                commands.push(((), WantsToAttack{ actor: *entity, victim: *npc_entity }));
            }
        } else {
            commands.push(((), WantsToMove{ actor: *entity, destination }));
        }
    }
}

fn patrol(
    entity: &Entity, 
    commands: &mut CommandBuffer,
    player_entity: &Entity, 
    pt: &Point, 
    tick: usize, 
    dir: &Direction, 
    map: &Map, 
    npcs: &HashMap<Point, Entity>,
) {
    let destination = match dir {
        Direction::North => Point::new( 0, -1) + *pt,
        Direction::East  => Point::new( 1,  0) + *pt,
        Direction::South => Point::new( 0,  1) + *pt,
        Direction::West  => Point::new(-1,  0) + *pt, 
    };

    if let Some(npc_entity) = npcs.get(&destination) {
        if npc_entity == player_entity {
            commands.push(((), WantsToAttack{ actor: *entity, victim: *npc_entity }));
        } else {
            commands.add_component(*entity, MoveStrategy::Patrol(tick, dir.next()));
        }
    } else {
        if map.can_enter(destination) {
            commands.push(((), WantsToMove{ actor: *entity, destination }));
        } else {
            commands.add_component(*entity, MoveStrategy::Patrol(tick, dir.next()));
        }
    }
}