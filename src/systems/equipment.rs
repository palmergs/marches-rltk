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

        println!("unequipping {:?} for {:?}", items_to_unequip, slot);
        for i in items_to_unequip.into_iter() {
            unequip_item(ecs, commands, player, i);
        }

        // equip the item
        commands.add_component(item, Equipped{ slot });

        // update stats based on the item
        if let Ok(item_stats) = ecs.entry_ref(item).unwrap().get_component::<Stats>() {
            let armor = item_stats.armor;
            let speed = item_stats.speed;
            let power = item_stats.power;
            if let Ok(mut player_stats) = ecs.entry_mut(player).unwrap().get_component_mut::<Stats>() {
                player_stats.armor += armor;
                player_stats.speed += speed;
                player_stats.power += power;
            }
        }

        // update the light source radius if it is 
        if let Ok(fol) = ecs.entry_ref(item).unwrap().get_component::<FieldOfLight>() {
            let radius = fol.radius;
            if let Ok(fol) = ecs.entry_ref(player).unwrap().get_component::<FieldOfLight>() {
                if radius > fol.radius {
                    commands.add_component(player, FieldOfLight::new(radius));
                }
            }
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
        if let Ok(mut player_stats) = ecs.entry_mut(player).unwrap().get_component_mut::<Stats>() {
            player_stats.armor -= armor;
            player_stats.speed -= speed;
            player_stats.power -= power;
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

    commands.remove_component::<Equipped>(item);
}

pub fn drop_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, item: Entity) {
    let (player, player_pt) = player_at(ecs);
    if let Ok(item_ref) = ecs.entry_ref(item) {
        if item_ref.get_component::<Equipped>().is_ok() {
            unequip_item(ecs, commands, player, item);
        }

        commands.remove_component::<Carried>(item);
        commands.add_component(item, player_pt);
    }
}

pub fn get_item(ecs: &SubWorld, commands: &mut CommandBuffer, item: Entity) {
    let (player, player_pt) = player_at(ecs);
    if let Ok(item_ref) = ecs.entry_ref(item) {
        commands.remove_component::<Point>(item);
        commands.add_component(item, Carried{})
    }
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

pub fn list_of_equipment<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity, EquipmentSlot)> {

    let mut vec = <(Entity, &Render, &Equipped)>::query()
        .iter(ecs)
        .map(|(entity, render, equipped)| (&render.name[..], *entity, equipped.slot))
        .collect::<Vec<_>>();

    vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    vec
}