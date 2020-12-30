use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfLight)]
pub fn fol(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut lights = <(&Point, &mut FieldOfLight)>::query();
    lights.iter_mut(ecs)
        .filter(|(_, fol)| fol.is_dirty )
        .for_each(|(pt, fol)| {
            fol.lit_tiles = field_of_view_set(*pt, fol.radius, map);
            fol.is_dirty = false;
        });
}