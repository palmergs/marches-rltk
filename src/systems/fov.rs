use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Player)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map,
) {
    let mut views = <(&Render, &mut FieldOfView)>::query();
    views.iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty )
        .for_each(|(render, fov)| {
            fov.visible_tiles = field_of_view_set(render.pt, fov.radius, map);
            fov.is_dirty = false;
        });
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