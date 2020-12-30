use crate::prelude::*;

mod player_input;
mod map_render;
mod actor_render;
mod item_render;
mod state_change;
mod movement;
mod fov;
mod fol;

pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(actor_render::actor_render_system())
        .add_system(item_render::item_render_system())
        .build()
}

pub fn build_player_schedule() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(actor_render::actor_render_system())
        .add_system(item_render::item_render_system())
        .flush()
        .add_system(state_change::state_change_system())
        .build()
}

pub fn build_computer_schedule() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(actor_render::actor_render_system())
        .add_system(item_render::item_render_system())
        .flush()
        .add_system(state_change::state_change_system())
        .build()
}