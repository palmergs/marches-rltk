use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Render)]
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
    let mut rng = Rng::new();
    let dmg = rng.range(1, 5);

    if let Ok(mut stats) = ecs.entry_mut(cmd.victim).unwrap().get_component_mut::<Stats>() {
        let (focus_dmg, vigor_dmg) = stats.focus.hit(dmg);
        stats.focus.curr -= focus_dmg;
        stats.vigor.curr -= vigor_dmg;
        let is_killed = stats.vigor.is_zero();
        let is_player = ecs.entry_ref(cmd.victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        if let Ok(render) = ecs.entry_ref(cmd.victim).unwrap().get_component::<Render>() {
            if is_killed {
                commands.push((Text{
                    display: TextDisplay::AnimateUp(render.pt),
                    text: format!("KILLED!").to_string(),
                    color: RGBA::from_f32(1., 0., 0., 1.),
                    ticks: 200,
                    count: 0,
                },));

                // TODO: for ease of access to pt, removing the entity here, but this
                // might cause problems in the future if an entity with Stats but not Render
                // is killed.
                if !is_player {

                    // TODO: this feels ugly by tightly linking the removal of a specific type 
                    // of entity to the map. 
                    if ecs.entry_ref(cmd.victim).unwrap().get_component::<Item>().is_ok() {
                        map.blocked.remove(&render.pt);
                        map.opaque.remove(&render.pt);
                    }

                    // Check to see if this entity spawns something when it is killed
                    if let Ok(spawns) = ecs.entry_ref(cmd.victim).unwrap().get_component::<Spawns>() {
                        spawns.entities.iter()
                            .filter(|se| se.trigger == SpawnTrigger::Killed )
                            .filter(|se| rng.range(0, 1000) < se.chance)
                            .for_each(|se| {
                                match &se.id[..] {
                                    "doormouse" => { 
                                        commands.push(doormouse_tuple(render.pt)); 
                                    },
                                    "skeleton" => { 
                                        commands.push(skeleton_tuple(render.pt)); 
                                    },
                                    _ => { println!("no entity spawned for {}", se.id); },
                                };
                            });
                    }

                    commands.remove(cmd.victim);
                    
                }
                map.actors.remove(&render.pt);
            } else {
                commands.push((Text{
                    display: TextDisplay::AnimateUp(render.pt),
                    text: format!("{}", dmg).to_string(),
                    color: RGBA::from_f32(1., 0., 0., 1.),
                    ticks: 50,
                    count: 0,
                },));
            }
        }
    }

    commands.remove(*entity);
}