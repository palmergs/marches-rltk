use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Item)]
#[read_component(Spawns)]
#[write_component(Stats)]
pub fn combat(
    entity: &Entity,
    cmd: &WantsToAttack,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map,
 ) {

    let player_entry = ecs.entry_ref(cmd.actor).unwrap();
    let attacker_power = player_entry.get_component::<Stats>().unwrap().power;

    if let Ok(mut victim_stats) = ecs.entry_mut(cmd.victim).unwrap().get_component_mut::<Stats>() {

        let mut rng = Rng::new();
        let dmg = std::cmp::max(0, rng.range(-2, 5) + attacker_power - victim_stats.armor);

        let (focus_dmg, vigor_dmg) = victim_stats.focus.hit(dmg);
        victim_stats.focus.curr -= focus_dmg;
        victim_stats.vigor.curr -= vigor_dmg;
        let is_killed = victim_stats.vigor.is_zero();
        let is_player = ecs.entry_ref(cmd.victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        if let Ok(pt) = ecs.entry_ref(cmd.victim).unwrap().get_component::<Point>() {
            if is_killed {
                commands.push((Text{
                    display: TextDisplay::AnimateUp(*pt),
                    text: format!("KILLED!").to_string(),
                    color: RGBA::from_f32(1., 0., 0., 1.),
                    ticks: 200,
                    count: 0,
                },));

                // TODO: for ease of access to pt, removing the entity here, but this
                // might cause problems in the future if an entity with Stats but not Point
                // is killed.
                if !is_player {

                    // TODO: this feels ugly by tightly linking the removal of a specific type 
                    // of entity to the map. 
                    if ecs.entry_ref(cmd.victim).unwrap().get_component::<Item>().is_ok() {
                        map.blocked.remove(pt);
                        map.opaque.remove(pt);
                    }

                    // Check to see if this entity spawns something when it is killed
                    if let Ok(spawns) = ecs.entry_ref(cmd.victim).unwrap().get_component::<Spawns>() {

                        // TODO: don't like this either, I shouldn't have to have match against 
                        // string ids here but I haven't figured out if there's a shared interface
                        // for both command buffer and world
                        spawns.entities.iter()
                            .for_each(|se| {
                                if se.trigger == SpawnTrigger::Killed && se.should_spawn(&mut rng) {
                                    match &se.id[..] {
                                        "doormouse" => { 
                                            commands.push(doormouse_tuple(*pt)); 
                                        },
                                        "skeleton" => { 
                                            commands.push(skeleton_tuple(*pt)); 
                                        },
                                        _ => { println!("no entity spawned for {}", se.id); },
                                    };
                                }
                            });
                    }

                    commands.remove(cmd.victim);
                    
                }
                map.actors.remove(pt);
            } else {
                let color = if dmg > 0 {
                    RGBA::from_f32(1., 0., 0., 1.)
                } else {
                    RGBA::from_f32(0.5, 0.5, 0.5, 1.)
                };

                commands.push((Text{
                    display: TextDisplay::AnimateUp(*pt),
                    text: format!("{}", dmg).to_string(),
                    color,
                    ticks: 50,
                    count: 0,
                },));
            }
        }
    }

    commands.remove(*entity);
}