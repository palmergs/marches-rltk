use crate::prelude::*;
use std::collections::HashMap;

pub fn list_of_items<'a>(ecs: &'a SubWorld) -> Vec<(&'a str, Entity, usize)> {

    // step one: merge all the like items so the count totals can be determined
    let mut map: HashMap<&str, (Entity, usize)> = HashMap::new();
    <(Entity, &Render, &Item)>::query()
        .filter(component::<Carried>())
        .iter(ecs)
        .filter(|(_, _, item)| item.can_get )
        .for_each(|(entity, render, _)| {
            if let Some(tuple) = map.get_mut(&render.name[..]) {
                (*tuple).1 += 1;
            } else {
                map.insert(&render.name[..], (*entity, 1));
            }
        });

    // step two: insert into the vector
    let mut vec = Vec::new();
    for (name, tuple) in map.iter() {
        vec.push((*name, tuple.0, tuple.1));
    }

    // step 3: sort by name
    vec.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    vec
}