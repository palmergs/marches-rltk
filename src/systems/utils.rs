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

