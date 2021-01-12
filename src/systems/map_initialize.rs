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

    <&Point>::query()
        .filter(component::<Actor>())
        .iter(ecs)
        .for_each(|pt| { map.actors.insert(*pt); });

    <(&Item, &Point)>::query()
        .iter(ecs)
        .for_each(|(item, pt)| {
            if item.opaque { map.opaque.insert(*pt); }
            if item.blocking { map.blocked.insert(*pt); }
        });
}