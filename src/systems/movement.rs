use crate::prelude::*;

#[system(for_each)]
#[read_component(WantsToMove)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    cmd: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,      
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,  
) {
    if map.can_enter(cmd.destination) {
        commands.add_component(cmd.actor, cmd.destination);
        if ecs.entry_ref(cmd.actor).unwrap().get_component::<Player>().is_ok() {   
            camera.on_player_move(cmd.destination);
        }
    }
    commands.remove(*entity);
}