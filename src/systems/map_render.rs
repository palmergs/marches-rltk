use crate::prelude::*;

use std::collections::HashMap;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] tick: &TickCount,
) {
    let mut fov = <(&Player, &Point, &FieldOfView)>::query();
    let (player, player_pt, player_fov) = fov.iter(ecs).next().unwrap();

    let mut fol = <(&Point, &FieldOfLight)>::query();
    let mut lighting: HashMap<Point, f32> = HashMap::new();
    fol.iter(ecs).filter(|(pt, fol)| {
        let distance = DistanceAlg::Pythagoras.distance2d(**pt, *player_pt) as f32;
        distance < (fol.radius + player_fov.radius) as f32
    }).for_each(|(pt, fol)| {
        for tile in fol.lit_tiles.iter() {
            let mut light = 1.0 - (DistanceAlg::Pythagoras.distance2d(*pt, *tile) as f32 / fol.radius as f32);
            if light < 0.1 { light = 0.1; }
            match lighting.get(tile) {
                Some(val) => if val > &light { lighting.insert(*tile, *val); },
                None => { lighting.insert(*tile, light); },
            }
        }
    });

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(FLOOR_LAYER);
    // draw_batch.cls();

    let offset = camera.offset();
    let mut fg = RGB::from_f32(1.0, 1.0, 1.0);
    for y in camera.top ..= camera.bottom {
        for x in camera.left ..= camera.right {
            let pt = Point::new(x, y);
            if map.in_bounds(pt) {
                let idx = map.point2d_to_index(pt);
                let screen_pt = pt - offset;
                let is_visible = player_fov.visible_tiles.contains(&pt);
                let is_remembered = map.revealed[idx];
                if is_visible || is_remembered {
                    if is_visible {
                        fg = match lighting.get(&pt) {
                            Some(val) => RGB::from_f32(*val, *val, *val),
                            None => RGB::from_f32(0.1, 0.1, 0.1),
                        };
                    } else {
                        fg = RGB::from_f32(0.1, 0.1, 0.1);
                    }

                    let color = ColorPair::new(fg, RGB::from_f32(0., 0., 0.));
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
    }
    draw_batch.submit(0).expect("batch error in map_render");
}

