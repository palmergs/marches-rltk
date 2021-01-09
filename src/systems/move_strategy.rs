use crate::prelude::*;

use std::collections::HashMap;

#[system]
#[read_component(Player)]
#[read_component(Actor)]
#[read_component(Point)]
#[read_component(FieldOfView)]
#[read_component(Stats)]
#[read_component(Mental)]
pub fn move_strategy(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] tick: &TickCount,
) {
    // points containing actors
    let mut npcs: HashMap<Point, Entity> = HashMap::new();
    <(Entity, &Point)>::query()
        .filter(component::<Actor>())
        .iter(ecs)
        .for_each(|(entity, pt)| { npcs.insert(*pt, *entity); });

    // location and entity for player
    let mut query = <(Entity, &Player, &Point)>::query();
    let (player_entity, _, player_pt) = query.iter(ecs).next().unwrap();

    // map path to player from nearby locations
    let player_idx = map.point2d_to_index(*player_pt);
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &search_targets,
        map,
        1024.0);

    let mut rng = Rng::new();
    let mut query = <(Entity, &Point, &FieldOfView, &Mental, &Stats)>::query();
    query.iter(ecs).for_each(|(entity, pt, fov, mental, stats)| {
        if tick.act(stats.speed) {
            let can_see = fov.visible_tiles.contains(&player_pt);
            let strategy = match mental.new_strategy(can_see) {
                Some(new_strat) => {
                    commands.push((WantsToChangeStrategy{ actor: *entity, strategy: new_strat }, ));
                    new_strat
                },
                None => mental.strategy
            };

            match strategy {
                MoveStrategy::Patrol(dir) => {
                    let destination = match dir {
                        Direction::North => Point::new( 0, -1) + *pt,
                        Direction::East  => Point::new( 1,  0) + *pt,
                        Direction::South => Point::new( 0,  1) + *pt,
                        Direction::West  => Point::new(-1,  0) + *pt,
                    };

                    if let Some(npc_entity) = npcs.get(&destination) {
                        if npc_entity == player_entity {
                            commands.push((WantsToAttack{ actor: *entity, victim: *npc_entity }, ));
                        } else {
                            commands.push((WantsToChangeStrategy{ actor: *entity, strategy: MoveStrategy::Patrol(dir.next())}, ));
                        }
                    } else {
                        if map.can_enter(destination) {
                            commands.push((WantsToMove{ actor: *entity, destination }, ));
                        } else {
                            commands.push((WantsToChangeStrategy{ actor: *entity, strategy: MoveStrategy::Patrol(dir.next())}, ));
                        }
                    }
                },

                MoveStrategy::Random => {
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

                    if let Some(npc_entity) = npcs.get(&destination) {
                        if npc_entity == player_entity {
                            commands.push((WantsToAttack{ actor: *entity, victim: *npc_entity }, ));
                        }
                    } else {
                        if map.can_enter(destination) {
                            commands.push((WantsToMove{ actor: *entity, destination }, ));
                        }
                    }
                },

                MoveStrategy::Chase => {
                    let entity_idx = map.point2d_to_index(*pt);
                    if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, entity_idx, map) {
                        let distance = DistanceAlg::Pythagoras.distance2d(*pt, *player_pt);
                        if distance > 1.2 {
                            let destination = map.index_to_point2d(destination);
                            if npcs.get(&destination).is_none() {
                                commands.push((WantsToMove{ actor: *entity, destination }, ));
                            }
                        } else {
                            commands.push((WantsToAttack{ actor: *entity, victim: *player_entity }, ));
                        };
                    }
                },

                MoveStrategy::Flee => {
                    let entity_idx = map.point2d_to_index(*pt);
                    if let Some(destination) = DijkstraMap::find_highest_exit(&dijkstra_map, entity_idx, map) {
                        let distance = DistanceAlg::Pythagoras.distance2d(*pt, *player_pt);
                        if distance > 1.2 {
                            let destination = map.index_to_point2d(destination);
                            if npcs.get(&destination).is_none() {
                                commands.push((WantsToMove{ actor: *entity, destination }, ));
                            }
                        } else {
                            commands.push((WantsToAttack{ actor: *entity, victim: *player_entity }, ));
                        };
                    }
                },

                MoveStrategy::Player => ()
            }
        }
    });
}
