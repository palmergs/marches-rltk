use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    cmd: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    #[resource] tick: &TickCount,
) {
    if map.can_enter(cmd.destination) {

        // Overwrite the Point component on the actor entity
        commands.add_component(cmd.actor, cmd.destination);

        if let Ok(entry) = ecs.entry_ref(cmd.actor) {

            // Update the circle of light around the moved entity
            if let Ok(fol) = entry.get_component::<FieldOfLight>() {
                commands.add_component(cmd.actor, fol.clone_dirty());
            }

            // Update the circle of vision around the moved entity
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(cmd.actor, fov.clone_dirty());

                // Move the camera if this is a Player
                if ecs.entry_ref(cmd.actor).unwrap().get_component::<Player>().is_ok() {
                    camera.on_player_move(cmd.destination);
                }
            }
        }
    }
    commands.remove(*entity);
}