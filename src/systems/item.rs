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
    let player = player_entity(ecs);
    if let Ok(entry) = ecs.entry_ref(cmd.item) {
        commands.add_component(cmd.item, Carried{ by: cmd.actor, equipped: false });

        // if you pick up a light source update the field of view
        if let Ok(fol) = ecs.entry_ref(cmd.item).unwrap().get_component::<FieldOfLight>() {
            commands.add_component(cmd.item, fol.clone_dirty());
            if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
                commands.add_component(player, fov.clone_dirty());
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
    let (player, player_pt) = player_at(ecs);

    // Remove the carried component
    commands.remove_component::<Carried>(cmd.item);
    commands.add_component(cmd.item, player_pt);
    unequip_item(ecs, commands, player, cmd.item);

    // if you drop a light source update the field of view
    if let Ok(fol) = ecs.entry_ref(cmd.item).unwrap().get_component::<FieldOfLight>() {
        commands.add_component(cmd.item, fol.clone_dirty());
        if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
            commands.add_component(player, fov.clone_dirty());
        }
    }

    commands.push((Text{
        display: TextDisplay::Fade(player_pt),
        text: format!("dropped").to_string(),
        color: RGBA::from_f32(0., 1., 0., 1.0),
        ticks: 40,
        count: 0,
    },));

    commands.remove(*entity);
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Equipped)]
#[read_component(Equippable)]
#[read_component(Carried)]
#[read_component(Point)]
#[read_component(Stats)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
pub fn equip(
    entity: &Entity,
    cmd: &WantsToEquip,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Ok(equippable) = ecs.entry_ref(cmd.item).unwrap().get_component::<Equippable>() {
        let player = player_entity(ecs);

        // step 1: check to see if the player is already using those slote
        let mut unequip_entities = Vec::new();
        <(Entity, &Equipped)>::query()
            .iter(ecs)
            .for_each(|(item, equipped)| {
                match equippable.primary {
                    EquipmentSlot::BothHands => {
                        match equipped.slot {
                            EquipmentSlot::BothHands | EquipmentSlot::LeftHand | EquipmentSlot::RightHand => {
                                unequip_entities.push(item);
                            },
                            _ => (),
                        }
                    },
                    _ => {
                        if equippable.primary == equipped.slot {
                            unequip_entities.push(item);
                        }
                    }
                }
            });

        // step 2: unequip any currently used slots
        for entity in unequip_entities {
            unequip_item(ecs, commands, player, *entity);
        }

        // step 3: equip the new item
        equip_item(ecs, commands, player, cmd.item, equippable.primary);
    }

    commands.remove(*entity);
}

fn equip_item(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    player: Entity,
    item: Entity,
    slot: EquipmentSlot,
) {
    println!("in equip item {:?}", slot);
    if let Ok(fol) = ecs.entry_ref(item).unwrap().get_component::<FieldOfLight>() {
        println!("found item field of light...");
        commands.add_component(player, fol.clone_dirty());
        if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
            println!("found player field of view...");
            commands.add_component(player, fov.clone_dirty());
        }
    }

    commands.add_component(item, Equipped{ slot });
}

fn unequip_item(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    player: Entity,
    item: Entity,
) {
    if let Ok(_) = ecs.entry_ref(item).unwrap().get_component::<FieldOfLight>() {
        if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
            commands.add_component(player, fov.clone_dirty());
        }
        commands.add_component(player, FieldOfLight::new(1).clone_dirty());
    }

    commands.remove_component::<Equipped>(item);
}

#[system(for_each)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Equipped)]
#[read_component(Carried)]
#[read_component(Point)]
#[read_component(Stats)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
pub fn unequip(
    entity: &Entity,
    cmd: &WantsToDrop,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Ok(equippable) = ecs.entry_ref(cmd.item).unwrap().get_component::<Equippable>() {
        let (player, player_pt) = player_at(ecs);
    }

    commands.remove(*entity);
}
