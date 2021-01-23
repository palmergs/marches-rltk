use crate::prelude::*;

pub fn equip_item(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer, 
    player: Entity, 
    item: Entity,
) {
    println!("in equip item...");
    if let Ok(equippable) = ecs.entry_ref(item).unwrap().get_component::<Equippable>() {
        let slot = equippable.primary;

        // remove any previously equipped items in the slot
        let mut query = <(Entity, &Equipped)>::query();
        let items_to_unequip = query
            .iter(ecs)
            .filter(|(_, eq)| {
                eq.slot == slot 
                    || (slot == EquipmentSlot::BothHands 
                        && (eq.slot == EquipmentSlot::LeftHand 
                            || eq.slot == EquipmentSlot::RightHand))
            })
            .map(|(entity, _)| *entity)
            .collect::<Vec<_>>();

        for i in items_to_unequip.into_iter() {
            unequip_item(ecs, commands, player, i);
        }

        // equip the item
        println!("equipping at {:?}", slot);
        commands.add_component(item, Equipped{ slot });

        // update stats based on the item
        if let Ok(item_stats) = ecs.entry_ref(item).unwrap().get_component::<Stats>() {
            let armor = item_stats.armor;
            let speed = item_stats.speed;
            let power = item_stats.power;
            if let Ok(player_stats) = ecs.entry_ref(player).unwrap().get_component::<Stats>() {
                let mut stats = player_stats.clone();
                stats.armor += armor;
                stats.speed += speed;
                stats.power += power;
                commands.add_component(player, stats);
            }
        }

        // update the light source radius if it is 
        if let Ok(fol) = ecs.entry_ref(item).unwrap().get_component::<FieldOfLight>() {
            let radius = fol.radius;
            if let Ok(fol) = ecs.entry_ref(player).unwrap().get_component::<FieldOfLight>() {
                if radius >= fol.radius {
                    commands.add_component(player, FieldOfLight::new(radius));
                }
            }
        }
        if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
            commands.add_component(player, fov.clone_dirty());
        }
    }
}

pub fn unequip_item(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer, 
    player: Entity, 
    item: Entity
) {
    // update stats based on item
    if let Ok(item_stats) = ecs.entry_ref(item).unwrap().get_component::<Stats>() {
        let armor = item_stats.armor;
        let speed = item_stats.speed;
        let power = item_stats.power;
        if let Ok(player_stats) = ecs.entry_ref(player).unwrap().get_component::<Stats>() {
            let mut stats = player_stats.clone();
                stats.armor -= armor;
                stats.speed -= speed;
                stats.power -= power;
                commands.add_component(player, stats);
        }
    }

    // get the current radius of any remaining light sources
    match <(Entity, &FieldOfLight)>::query()
        .filter(component::<Equipped>())
        .iter(ecs)
        .filter(|(e, _)| **e != item)
        .map(|(_, fol)| fol.radius )
        .max() 
    {
        Some(radius) => commands.add_component(player, FieldOfLight::new(radius)),
        _ => commands.add_component(player, FieldOfLight::new(1)),
    }
    if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
        commands.add_component(player, fov.clone_dirty());
    }

    commands.remove_component::<Equipped>(item);
}

pub fn drop_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, item: Entity) {
    let (_, player_pt) = player_at(ecs);
    drop_item_at(ecs, commands, item, player_pt);
}

pub fn drop_item_at(ecs: &mut SubWorld, commands: &mut CommandBuffer, item: Entity, pt: Point) {
    let (player, _) = player_at(ecs);
    println!("in drop item {:?} at {:?}", item, pt);
    if let Ok(item_ref) = ecs.entry_ref(item) {
        if item_ref.get_component::<Equipped>().is_ok() {
            println!("about to un equip");
            unequip_item(ecs, commands, player, item);
        }

        println!("about to remove component carried");
        commands.remove_component::<Carried>(item);

        println!("about to add component {:?}={:?}", item, pt);
        commands.add_component(item, pt);

        // if this is a light source, let the world know that it needs ot 
        // update the lit tiles.
        if let Ok(fol) = ecs.entry_ref(item).unwrap().get_component::<FieldOfLight>() {
            commands.add_component(item, fol.clone_dirty());
            if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
                commands.add_component(player, fov.clone_dirty());
            }
        }
        commands.push((Text{
            display: TextDisplay::Fade(pt),
            text: format!("dropped").to_string(),
            color: RGBA::from_f32(0., 1., 0., 1.0),
            ticks: 40,
            count: 0,
        },));
    } else {
        println!("could not get entry ref");
    }
}

pub fn get_item(ecs: &SubWorld, commands: &mut CommandBuffer, item: Entity) {
    let (player, player_pt) = player_at(ecs);

    commands.remove_component::<Point>(item);
    commands.add_component(item, Carried{});

    // if this is a light source, let the world know that it needs ot 
    // update the lit tiles.
    if let Ok(fol) = ecs.entry_ref(item).unwrap().get_component::<FieldOfLight>() {
        commands.add_component(item, fol.clone_dirty());
        if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
            commands.add_component(player, fov.clone_dirty());
        }
    }

    commands.push((Text{
        display: TextDisplay::Fade(player_pt),
        text: format!("got it").to_string(),
        color: RGBA::from_f32(0., 1., 0., 1.0),
        ticks: 40,
        count: 0,
    },));
}

pub fn list_of_items<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity)> {
    let mut vec = <(Entity, &Render)>::query()
        .filter(component::<Carried>())
        .iter(ecs)
        .map(|(entity, render)| (&render.name[..], *entity))
        .collect::<Vec<_>>();

    vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    vec
}

pub fn list_of_item_counts<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity, usize)> {
    let mut vec = Vec::<(&'a str, Entity, usize)>::new();
    for (name, entity) in list_of_items(ecs) {
        let len = vec.len();
        if len == 0 || vec[len - 1].0 != name {
            vec.push((name, entity, 1));
        } else {
            vec[len - 1].2 += 1;
        }
    }
    vec
}

pub fn player_weapon(ecs: &SubWorld) -> Option<Entity> {
    let mut query = <(Entity, &Equipped)>::query();
    if let Some((entity, _)) = query.iter(ecs)
        .filter(|(_, e)| e.slot == EquipmentSlot::RightHand || e.slot == EquipmentSlot::BothHands )
        .next() {

           return Some(*entity);
    }
    
    if let Some((entity, _)) = query.iter(ecs)
        .filter(|(_, e)| e.slot == EquipmentSlot::LeftHand )
        .next() {

            return Some(*entity);
    }

    None
}

pub fn list_of_equipment<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity, EquipmentSlot)> {

    let mut vec = <(Entity, &Render, &Equipped)>::query()
        .iter(ecs)
        .map(|(entity, render, equipped)| (&render.name[..], *entity, equipped.slot))
        .collect::<Vec<_>>();

    vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    vec
}
