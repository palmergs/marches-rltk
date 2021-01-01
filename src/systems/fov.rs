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