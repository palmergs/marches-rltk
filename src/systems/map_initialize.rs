use crate::prelude::*;

#[system]
#[read_component(Actor)]
#[read_component(Item)]
#[read_component(Render)]
#[read_component(Opaque)]
#[read_component(Blocking)]
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

    let mut query = <(&Item, &Render)>::query().filter(component::<Opaque>());
    query.iter(ecs)
        .for_each(|(_, render)| { map.opaque.insert(render.pt); });

    let mut query = <(&Item, &Render)>::query().filter(component::<Blocking>());
    query.iter(ecs)
        .for_each(|(_, render)| { map.blocked.insert(render.pt); });
}