use crate::prelude::*;

mod player_input;
mod render;
mod state_change;
mod random_movers;
mod movement;
mod might_talk;
mod fov;
mod fol;
mod fading_text;

pub fn build_input_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(fading_text::fading_text_system())
        .build()
}

pub fn build_player_schedule() -> Schedule {
    Schedule::builder()
        .add_system(random_movers::random_movers_system())
        .flush()
        .add_system(might_talk::might_talk_system())
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(fading_text::fading_text_system())
        .flush()
        .add_system(state_change::state_change_system())
        .build()
}

pub fn build_computer_schedule() -> Schedule {
    Schedule::builder()
        .add_system(random_movers::random_movers_system())
        .flush()
        .add_system(might_talk::might_talk_system())
        .add_system(movement::movement_system())
        .flush()
        .add_system(fov::fov_system())
        .add_system(fol::fol_system())
        .flush()
        .add_system(render::render_system())
        .add_system(fading_text::fading_text_system())
        .flush()
        .add_system(state_change::state_change_system())
        .build()
}