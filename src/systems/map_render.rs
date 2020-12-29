use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(FLOOR_LAYER);
    draw_batch.cls();

    let offset = camera.offset();
    let color = ColorPair::new(WHITE, BLACK);
    for y in camera.top ..= camera.bottom {
        for x in camera.left ..= camera.right {
            let pt = Point::new(x, y);
            let idx = map.point2d_to_index(pt);
            if map.in_bounds(pt) {
                let screen_pt = pt - offset;
                match map.tiles[idx] {
                    TileType::Floor => draw_batch.set(screen_pt, color, 128 + 18),
                    TileType::Wall => draw_batch.set(screen_pt, color, 128 + 19),
                    TileType::Tree => draw_batch.set(screen_pt, color, 21),
                    TileType::Door => draw_batch.set(screen_pt, color, 2),
                    TileType::DoorOpen => draw_batch.set(screen_pt, color, 3),
                    TileType::Chest => draw_batch.set(screen_pt, color, 18),
                    TileType::ChestEmpty => draw_batch.set(screen_pt, color, 19),
                    TileType::Bookshelf => draw_batch.set(screen_pt, color, 25),
                };
            }
        }
    }
    draw_batch.submit(0).expect("batch error in map_render");
}