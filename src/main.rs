#[macro_use]
extern crate lazy_static;
extern crate bracket_lib;
extern crate serde;

mod components;
mod maps;
mod map_builders;
mod systems;
mod spawners;
mod cameras;
mod state;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub use RandomNumberGenerator as Rng;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use crate::maps::*;
    pub use crate::map_builders::*;
    pub use crate::components::*;
    pub use crate::systems::utils::*;
    pub use crate::systems::equipment::*;
    pub use crate::systems::*;
    pub use crate::spawners::*;
    pub use crate::cameras::*;
    pub use crate::state::*;
    pub use crate::tile_index;

    pub const SCREEN_WIDTH: i32 = 40;
    pub const SCREEN_HEIGHT: i32 = 25;
    pub const MAP_WIDTH: usize = 160;
    pub const MAP_HEIGHT: usize = 100;
    pub const MAP_TILES: usize = MAP_WIDTH * MAP_HEIGHT;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH * 4;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT * 4;

    // Terminal layers
    pub const FLOOR_LAYER:usize = 0;
    pub const ITEM_LAYER:usize = 1;
    pub const ACTOR_LAYER:usize = 2;
    pub const UI_LAYER:usize = 3;
}

use prelude::*;

/// tile_index is a utility method to reference tiles using 1-based indexing
pub fn tile_index(row: usize, col: usize) -> usize {
    ((row - 1) * 128) + (col - 1)
}

fn main() -> BError {
    load_actors();
    load_items();
    let context = BTermBuilder::new()
        .with_title("The Western Marches")
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("unicode_trunc_graph_32x32.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "unicode_trunc_graph_32x32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "unicode_trunc_graph_32x32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "unicode_trunc_graph_32x32.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
