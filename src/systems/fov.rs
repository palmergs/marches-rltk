use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map,
) {
    let mut views = <(&Point, &mut FieldOfView)>::query();
    views.iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty )
        .for_each(|(pt, fov)| {
            fov.visible_tiles = field_of_view_set(*pt, fov.radius, map);
            fov.visited_tiles = field_of_view_set(*pt, fov.radius / 2, map);
            fov.is_dirty = false;
        });

    let mut player_fov = <(&Player, &FieldOfView)>::query();
    let (_, fov) = player_fov.iter(ecs).next().unwrap();
    for pt in fov.visited_tiles.iter() {
        let idx = map.point2d_to_index(*pt);
        map.revealed[idx] = true;
    }
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