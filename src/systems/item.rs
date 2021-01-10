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
    if let Ok(entry) = ecs.entry_ref(cmd.item) {
        commands.add_component(cmd.item, Carried{ by: cmd.actor, equipped: false });

        if let Ok(pt) = entry.get_component::<Point>() {
            commands.remove_component::<Point>(cmd.item);
            commands.push((Text{
                display: TextDisplay::Fade(*pt),
                text: format!("got it").to_string(),
                color: RGBA::from_f32(0., 1., 0., 1.0),
                ticks: 200,
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
pub fn drop(
    entity: &Entity,
    cmd: &WantsToDrop,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
) {
    let (player_entity, player_pt) = <(Entity, &Player)>::query()
        .iter(ecs)
        .next()
        .unwrap();

    if let Ok(entry) = ecs.entry_ref(cmd.item) {

        // Remove the carried component
        commands.remove_component::<Carried>(cmd.item);
        commands.remove_component::<Equipped>(cmd.item);
        commands.add_component(cmd.item, *player_pt);


    }
}

pub fn list_of_items<'a>(ecs: &SubWorld, player: Entity) -> Vec<(usize, &'a str, Entity)> {
    let mut vec = Vec::new();

    vec
}