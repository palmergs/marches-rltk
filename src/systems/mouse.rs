use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(FieldOfView)]
pub fn mouse(
    ecs: &SubWorld,
    #[resource] mouse_pt: &Point,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    let (player, player_pt) = player_at(ecs);
    if let Ok(fov) = ecs.entry_ref(player).unwrap().get_component::<FieldOfView>() {
        let mouse_pt = *mouse_pt + camera.offset();
        if map.in_bounds(mouse_pt) && fov.visible_tiles.contains(&mouse_pt) {
            let mut draw_batch = DrawBatch::new();
            draw_batch.target(FLOOR_LAYER);
            let line = Bresenham::new(mouse_pt, player_pt);
            let color = ColorPair::new(RGB::from_f32(0.5, 0.5, 0.9), BLACK);
            for pt in line.into_iter() {
                let screen_pt = pt - camera.offset();
                let map_idx = map.point2d_to_index(pt);
                draw_batch.set(screen_pt, color, map.font_idx(map_idx));
            }
            draw_batch.submit(500).expect("batch error in render mouse");
        }
    }
}