use crate::prelude::*;

#[system]
#[read_component(Actor)]
#[read_component(Item)]
#[read_component(Point)]
pub fn map_initialize(
    ecs: &SubWorld,
    #[resource] map: &mut Map,
) {
    map.actors.clear();
    map.blocked.clear();
    map.opaque.clear();

    let mut query = <(&Actor, &Point)>::query();
    query.iter(ecs)
        .for_each(|(_, pt)| { map.actors.insert(*pt); });

    let mut query = <(&Item, &Point)>::query();
    query.iter(ecs)
        .for_each(|(item, pt)| {
            if item.opaque { map.opaque.insert(*pt); }
            if item.blocking { map.blocked.insert(*pt); }
        });
}