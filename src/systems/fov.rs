use crate::prelude::*;

use std::collections::HashSet;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(FieldOfLight)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map,
) {
    
    // Build a set of all the tiles that are illuminated in the map
    let mut illuminated_tiles = HashSet::new();
    let mut query_light = <(&Point, &FieldOfLight)>::query();
    query_light.iter(ecs)
        .for_each(|(pt, fol)| {
            illuminated_tiles.extend(field_of_view_set(*pt, fol.radius, map));
            // for pt in field_of_view_set(*pt, fol.radius, map).into_iter() {
            //     illuminated_tiles.insert(pt);
            // }
        });

    // The viewable tiles is the intersection between the actors
    // view radius and the set of illuminated tiles
    let mut query_view = <(&Point, &mut FieldOfView)>::query();
    query_view.iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty )
        .for_each(|(pt, fov)| {
            let mut new_set = HashSet::new();
            for pt in illuminated_tiles.intersection(&field_of_view_set(*pt, fov.radius, map)).into_iter() {
                new_set.insert(*pt);
            }
            fov.visible_tiles = new_set;
            fov.is_dirty = false;
        });


    // // Old algorithm that only shows visible tiles based on the 
    // // view field of the actor
    // let mut views = <(&Point, &mut FieldOfView)>::query();
    // views.iter_mut(ecs)
    //     .filter(|(_, fov)| fov.is_dirty )
    //     .for_each(|(pt, fov)| {
    //         fov.visible_tiles = field_of_view_set(*pt, fov.radius, map);
    //         fov.is_dirty = false;
    //     });
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn test_edge_appears_in_view() {
        let map = Map::new(0);
        let fov = field_of_view(Point::new(2, 2), 10, &map);
        let pt_left = Point::new(0, 2);
        let pt_top  = Point::new(2, 0);  

        assert!(fov.contains(&pt_left));
        assert!(fov.contains(&pt_top));
    }
}