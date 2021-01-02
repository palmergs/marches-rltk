use crate::prelude::*;
use std::collections::HashMap;

#[system]
#[read_component(Point)]
#[read_component(Actor)]
#[write_component(PatrolMover)]
pub fn patrol_movers(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] tick: &TickCount,
) {
    let mut npcs: HashMap<Point, Entity> = HashMap::new();
    <(Entity, &Point)>::query()
        .filter(component::<Actor>())
        .iter(ecs)
        .for_each(|(entity, pt)| { npcs.insert(*pt, *entity); });

    let mut query = <(Entity, &Point, &PatrolMover)>::query();
    query.iter(ecs).for_each(|(entity, pt, pm)| {
        let destination = match pm.0 {
            Direction::North => Point::new( 0, -1) + *pt,
            Direction::East  => Point::new( 1,  0) + *pt,
            Direction::South => Point::new( 0,  1) + *pt,
            Direction::West  => Point::new(-1,  0) + *pt, 
        };

        if let Some(npc_entity) = npcs.get(&destination) {
            commands.push(((), WantsToAttack{ actor: *entity, victim: *npc_entity }));
        } else {
            if map.can_enter(destination) {
                commands.push(((), WantsToMove{ actor: *entity, destination }));
            } else {
                commands.add_component(*entity, pm.next());
            }
        }
    });
}