use crate::prelude::*;

mod player_input;
mod render;
mod state_change;
mod move_strategy;
mod movement;
mod might_talk;
mod combat;
mod fov;
mod fol;
mod display_text;

// Initializing the map before the first render and player input
pub fn build_initialize_schedule() -> Schedule {
    Schedule::builder()
        .add_system(map_initialize::map_initialize_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(state_change::state_change_system())
        .build()
}

// Awaiting keyboard input and creating new player commands
pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(display_text::display_text_system())
        .build()
}

// Resolving movement and creating new npc commands
pub fn build_computer_schedule() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(move_strategy::move_strategy_system())
        .add_system(might_talk::might_talk_system())
        .flush()
        .add_system(render::render_system())
        .add_system(display_text::display_text_system())
        .add_system(state_change::state_change_system())
        .build()
}