use crate::prelude::*;
use std::collections::HashMap;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Actor)]
#[read_component(Item)]
#[read_component(Player)]
#[read_component(FieldOfView)]
#[read_component(FieldOfLight)]
pub fn render(
    ecs: &SubWorld,
    #[resource] camera: &Camera,
    #[resource] map: &Map,
) {
    // Initialize drawing batch
    let mut draw_batch = DrawBatch::new();

    // Camera and Player Info
    let camera_offset = camera.offset();
    let mut query = <(&Player, &FieldOfView)>::query();
    let (_, player_fov) = query.iter(ecs).next().unwrap();
    let visible_tiles = &player_fov.visible_tiles;

    // Lighting Info
    let mut tile_light: HashMap<Point, f32> = HashMap::new();
    let mut query = <(&Point, &FieldOfLight)>::query();
    query.iter(ecs)
        .filter(|(pt, _)| camera.in_view(**pt) )
        .for_each(|(pt, fol)| {
            for tile in fol.lit_tiles.iter() {
                let light = light_at_tile(*pt, fol.radius, *tile);
                match tile_light.get(tile) {
                    Some(val) => {
                        let mut light = light + *val;
                        if light > 1.0 { light = 1.0; }
                        tile_light.insert(*tile, light);
                    },
                    None => { tile_light.insert(*tile, light); },
                }
            }
        });

    // Draw Map
    draw_batch.target(FLOOR_LAYER);

    let bg = RGBA::from_f32(0.0, 0.0, 0.0, 0.0);
    for y in camera.top ..= camera.bottom {
        for x in camera.left ..= camera.right {
            let map_pt = Point::new(x, y);
            if map.in_bounds(map_pt) {
                let map_idx = map.point2d_to_index(map_pt);
                let screen_pt = map_pt - camera_offset;
                let is_visible = visible_tiles.contains(&map_pt);
                let is_remembered = map.revealed[map_idx];
                match lighting_at(map_pt, is_visible, is_remembered, &tile_light) {
                    Some(fg) => {
                        draw_batch.set(
                            screen_pt,
                            ColorPair::new(RGB::from_f32(fg, fg, fg), bg),
                            map.font_idx(map_idx));
                    },
                    None => ()
                }
            }
        }
    }

    // Draw Items
    draw_batch.target(ITEM_LAYER);

    let mut query = <(&Item, &Point, &Render)>::query();
    query.iter(ecs)
        .for_each(|(_, pt, render)| {
        if map.in_bounds(*pt) {
            let screen_pt = *pt - camera_offset;
            let is_visible = visible_tiles.contains(pt);
            match lighting_at(*pt, is_visible, false, &tile_light) {
                Some(fg) => {
                    draw_batch.set(
                        screen_pt,
                        ColorPair::new(RGBA::from_f32(fg, fg, fg, 1.0), bg),
                        render.tile);
                },
                None => ()
            }
        }
    });

    // Draw Actors
    draw_batch.target(ACTOR_LAYER);

    let mut query = <(&Actor, &Point, &Render)>::query();
    query.iter(ecs).for_each(|(_, pt, render)| {
        if map.in_bounds(*pt) {
            let screen_pt = *pt - camera_offset;
            let is_visible = visible_tiles.contains(pt);
            match lighting_at(*pt, is_visible, false, &tile_light) {
                Some(fg) => {
                    draw_batch.set(
                        screen_pt,
                        ColorPair::new(RGBA::from_f32(fg, fg, fg, 1.0), bg),
                        render.tile);
                },
                None => ()
            }
        }
    });

    // Submit drawing to context
    draw_batch.submit(0).expect("batch error in render");
}

#[inline]
fn lighting_at(
    pt: Point,
    visible: bool,
    remembered: bool,
    tile_light: &HashMap::<Point, f32>
) -> Option<f32> {
    if visible {
        match tile_light.get(&pt) {
            Some(val) => return Some(*val),
            None => return Some(0.2),
        }
    }

    if remembered {
        return Some(0.1);
    }

    None
}

#[inline]
fn light_at_tile(source: Point, radius: i32, tile: Point) -> f32 {
    let val = 1.0 - (DistanceAlg::Pythagoras.distance2d(source, tile) / radius as f32);
    if val < 0.2 { return 0.2; }
    val
}
