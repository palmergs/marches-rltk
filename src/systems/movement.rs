use crate::prelude::*;

#[system(for_each)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
#[read_component(Player)]
#[read_component(Item)]
#[write_component(Render)]
pub fn movement(
    entity: &Entity,
    cmd: &WantsToMove,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
) {
    if map.can_enter(cmd.destination) {

        // See if this entity has a light and view field
        if let Ok(mut entry) = ecs.entry_mut(cmd.actor) {

            // Update the render position
            let mut position_updated = false;
            if let Ok(render) = entry.get_component_mut::<Render>() {

                // this block is the final arbiter to determine if
                // a position is taking up a square
                if !map.actors.contains(&cmd.destination) && !map.blocked.contains(&cmd.destination) {
                    map.actors.remove(&render.pt);
                    render.pt = cmd.destination;
                    map.actors.insert(render.pt);
                    position_updated = true;
                }
            }

            if position_updated {
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
    }
    commands.remove(*entity);
}