use crate::prelude::*;

use std::collections::HashMap;

#[system]
#[read_component(Actor)]
#[read_component(Player)]
#[read_component(Outlook)]
#[read_component(WantsToMove)]
#[write_component(Render)]
#[write_component(FieldOfLight)]
#[write_component(FieldOfView)]
pub fn position_resolution(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    #[resource] tick: &TickCount,
) {
    // Keep track of the current location of npcs; if one moves, update the map
    let mut query = <(Entity, &Render)>::query().filter(component::<Actor>());
    let mut npcs: HashMap<Point, Entity> = HashMap::new();
    query.iter(ecs)
        .for_each(|(entity, render)| {
            npcs.insert(render.pt, *entity);
        });

    let mut query = <(&WantsToMove, &mut Render)>::query();
    query.iter_mut(ecs)
        .for_each(|(cmd, render)| {
            if map.can_enter(cmd.destination) {
                if npcs.get(&cmd.destination).is_none() {

                    npcs.remove(&render.pt);
                    render.pt = cmd.destination;
                    npcs.insert(cmd.destination, cmd.actor);

                    // See if this entity has a light and view field
                    if let Ok(entry) = ecs.entry_mut(cmd.actor) {

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
        });
}