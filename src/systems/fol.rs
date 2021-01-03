use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Player)]
#[write_component(FieldOfLight)]
pub fn fol(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map,
) {
    let mut lights = <(&Render, &mut FieldOfLight)>::query();
    lights.iter_mut(ecs)
        .filter(|(_, fol)| fol.is_dirty )
        .for_each(|(render, fol)| {
            fol.lit_tiles = field_of_view_set(render.pt, fol.radius, map);
            fol.is_dirty = false;
        });

    let mut player_fol = <&FieldOfLight>::query().filter(component::<Player>());
    if let Some(fov) = player_fol.iter(ecs).next() {
        for pt in fov.lit_tiles.iter() {
            let idx = map.point2d_to_index(*pt);
            map.revealed[idx] = true;
        }
    }
}