use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
#[read_component(Player)]
#[write_component(Render)]
pub fn movement(
    entity: &Entity,
    cmd: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    #[resource] _tick: &TickCount,
) {
    if map.can_enter(cmd.destination) {

        // // Overwrite the Point component on the actor entity
        // commands.add_component(cmd.actor, cmd.destination);

        // See if this entity has a light and view field
        if let Ok(mut entry) = ecs.entry_mut(cmd.actor) {

            // Update the render position
            if let Ok(render) = entry.get_component_mut::<Render>() {
                render.pt = cmd.destination;
            }

            // Update the circle of light around the moved entity
            if let Ok(fol) = entry.get_component::<FieldOfLight>() {
                commands.add_component(cmd.actor, fol.clone_dirty());
            }

            // Update the circle of vision around the moved entity
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(cmd.actor, fov.clone_dirty());

                // Move the camera if this is a Player
                if ecs.entry_ref(cmd.actor).unwrap().get_component::<Player>().is_ok() {
                    println!("player at {:?}", cmd.destination);
                    camera.on_player_move(cmd.destination);
                }
            }
        }
    }
    commands.remove(*entity);
}