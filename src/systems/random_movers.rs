use crate::prelude::*;
use std::collections::HashMap;

#[system]
#[read_component(Point)]
#[read_component(Actor)]
#[read_component(RandomMover)]
pub fn random_movers(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] tick: &TickCount,
) {
    let mut npcs: HashMap<Point, Entity> = HashMap::new();
    <(Entity, &Point)>::query()
        .filter(component::<Actor>())
        .iter(ecs)
        .for_each(|(entity, pt)| { npcs.insert(*pt, *entity); });

    let mut query = <(Entity, &Point, &RandomMover)>::query();
    let mut rng = Rng::new();
    query.iter(ecs).for_each(|(entity, pt, rm)| {
        if tick.act(rm.0) {
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
                    commands.push(((), WantsToAttack{ actor: *entity, victim: *npc_entity }));
                } else {
                    commands.push(((), WantsToMove{ actor: *entity, destination }));
                }
            }
        }
    });
}