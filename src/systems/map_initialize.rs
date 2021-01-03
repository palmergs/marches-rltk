use crate::prelude::*;

#[system]
#[read_component(Actor)]
#[read_component(Item)]
#[read_component(Render)]
pub fn map_initialize(
    ecs: &SubWorld,
    #[resource] map: &mut Map,
) {
    map.actors.clear();
    map.blocked.clear();
    map.opaque.clear();

    let mut query = <(&Actor, &Render)>::query();
    query.iter(ecs)
        .for_each(|(_, render)| { map.actors.insert(render.pt); });

    let mut query = <(&Item, &Render)>::query();
    query.iter(ecs)
        .for_each(|(item, render)| {
            if item.opaque { map.opaque.insert(render.pt); }
            if item.blocking { map.blocked.insert(render.pt); }
        });
}