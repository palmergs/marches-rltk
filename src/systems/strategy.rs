use crate::prelude::*;

#[system(for_each)]
#[write_component(Mental)]
pub fn strategy(
    entity: &Entity,
    cmd: &WantsToChangeStrategy,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
 ) {
    if let Ok(mut mental) = ecs.entry_mut(cmd.actor).unwrap().get_component_mut::<Mental>() {
        mental.strategy = cmd.strategy;
    }
    commands.remove(*entity);
 }