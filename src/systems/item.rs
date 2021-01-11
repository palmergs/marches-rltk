use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Item)]
#[write_component(Point)]
pub fn pickup(
    entity: &Entity,
    cmd: &WantsToGet,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let player = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    if let Ok(entry) = ecs.entry_ref(cmd.item) {
        commands.add_component(cmd.item, Carried{ by: cmd.actor, equipped: false });

        // if you pick up a light source update the field of view
        if let Ok(fol) = ecs.entry_ref(cmd.item).unwrap().get_component::<FieldOfLight>() {
            commands.add_component(cmd.item, fol.clone_dirty());
            if let Ok(fov) = ecs.entry_ref(*player).unwrap().get_component::<FieldOfView>() {
                commands.add_component(*player, fov.clone_dirty());
            }
        }

        if let Ok(pt) = entry.get_component::<Point>() {
            commands.remove_component::<Point>(cmd.item);
            commands.push((Text{
                display: TextDisplay::Fade(*pt),
                text: format!("got it").to_string(),
                color: RGBA::from_f32(0., 1., 0., 1.0),
                ticks: 40,
                count: 0,
            },));
        }
    }

    commands.remove(*entity);
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Equipped)]
#[read_component(Carried)]
#[read_component(Point)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
pub fn drop(
    entity: &Entity,
    cmd: &WantsToDrop,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let (player, player_pt) = <(Entity, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    // Remove the carried component
    commands.remove_component::<Carried>(cmd.item);
    commands.remove_component::<Equipped>(cmd.item);
    commands.add_component(cmd.item, *player_pt);

    // if you drop a light source update the field of view
    if let Ok(fol) = ecs.entry_ref(cmd.item).unwrap().get_component::<FieldOfLight>() {
        commands.add_component(cmd.item, fol.clone_dirty());
        if let Ok(fov) = ecs.entry_ref(*player).unwrap().get_component::<FieldOfView>() {
            commands.add_component(*player, fov.clone_dirty());
        }
    }

    commands.push((Text{
        display: TextDisplay::Fade(*player_pt),
        text: format!("dropped").to_string(),
        color: RGBA::from_f32(0., 1., 0., 1.0),
        ticks: 40,
        count: 0,
    },));

    commands.remove(*entity);
}
