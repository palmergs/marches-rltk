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

    let offset = camera.offset();
    for y in camera.top ..= camera.bottom {
        for x in camera.left ..= camera.right {
            let pt = Point::new(x, y);
            let idx = map.point2d_to_index(pt);
            if map.in_bounds(pt) {
                let color = ColorPair::new(WHITE, BLACK);
                match map.tiles[idx] {
                    TileType::Floor => draw_batch.set(pt - offset, color, 0),
                    TileType::Wall => draw_batch.set(pt - offset, color, 7),
                    TileType::Tree => draw_batch.set(pt - offset, color, 43),
                    TileType::Door => draw_batch.set(pt - offset, color, 2),
                    TileType::DoorOpen => draw_batch.set(pt - offset, color, 3),
                    TileType::Chest => draw_batch.set(pt - offset, color, 18),
                    TileType::ChestEmpty => draw_batch.set(pt - offset, color, 19),
                    TileType::Bookshelf => draw_batch.set(pt - offset, color, 47),
                };
            }
        }
    }
    draw_batch.submit(0).expect("batch error in map_render")
}