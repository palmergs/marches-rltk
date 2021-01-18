use crate::prelude::*;

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

pub fn player_view_dirty(ecs: &SubWorld, commands: &mut CommandBuffer, player: Entity) {
    if let Ok(fol) = ecs.entry_ref(player).unwrap().get_component::<FieldOfLight>() {
        commands.add_component(player, fol.clone_dirty());
    }

    if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
        commands.add_component(player, fov.clone_dirty());
    }
}

