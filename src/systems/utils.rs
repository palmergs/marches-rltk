use crate::prelude::*;
use std::collections::HashMap;

pub fn player_entity(ecs: &SubWorld) -> Entity {
    *<Entity>::query().filter(component::<Player>()).iter(ecs).next().unwrap()
}

pub fn player_stats<'a>(ecs: &'a SubWorld) -> &'a Stats {
    <&Stats>::query().filter(component::<Player>()).iter(ecs).next().unwrap()
}

pub fn player_at(ecs: &SubWorld) -> (Entity, Point) {
    let (entity, pt) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    (*entity, *pt)
}

pub fn player_info<'a>(ecs: &'a SubWorld) -> (&'a Entity, &'a Point, &'a Stats, &'a Physical, &'a Mental) {
    <(Entity, &Point, &Stats, &Physical, &Mental)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap()
}

// Get a vector of tuples that represents the things carried by the player.
pub fn list_of_items<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity, usize)> {

    // step one: merge all the like items so the count totals can be determined
    let mut map: HashMap<&str, (Entity, usize)> = HashMap::new();
    <(Entity, &Render, &Item)>::query()
        .filter(component::<Carried>())
        .iter(ecs)
        .filter(|(_, _, item)| item.can_get )
        .for_each(|(entity, render, _)| {
            if let Some(tuple) = map.get_mut(&render.name[..]) {
                (*tuple).1 += 1;
            } else {
                map.insert(&render.name[..], (*entity, 1));
            }
        });

    // step two: insert into the vector
    let mut vec = Vec::new();
    for (name, tuple) in map.iter() {
        vec.push((*name, tuple.0, tuple.1));
    }

    // step 3: sort by name
    vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    vec
}

pub fn list_of_equipment<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity, EquipmentSlot)> {

    let mut query = <(Entity, &Render, &Equipped)>::query();
    let mut vec = query
        .iter(ecs)
        .map(|(entity, render, equipped)| (&render.name[..], *entity, equipped.slot))
        .collect::<Vec<_>>();

    vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    vec
}