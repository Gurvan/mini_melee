use super::{Character, CharacterActionStates, CharacterAttributes, CharacterFrameData};
pub mod wait;
pub mod kneebend;
pub mod jump;
pub mod fall;
use wait::WAIT;
use kneebend::KNEEBEND;
use jump::{JUMPF, JUMPB};
use fall::FALL;


pub const ACTIONSTATE: &CharacterActionStates = &CharacterActionStates {
    WAIT: WAIT,
    KNEEBEND: KNEEBEND,
    JUMPF: JUMPF,
    JUMPB: JUMPB,
    FALL: FALL,
};

pub const FRAMEDATA: &CharacterFrameData = &CharacterFrameData {
    WAIT: 120,
    DASH: 21,
    RUN: 25,
    RUNBRAKE : 18,
    RUNTURN : 20,
    WALK: 26,
    JUMPF: 40,
    JUMPB: 40,
    FALL: 8,
    FALLAERIAL: 8,
    FALLSPECIAL: 8,
    JUMPAERIALF: 50,
    JUMPAERIALB: 50,
};

pub const ATTRIBUTE: &CharacterAttributes = &CharacterAttributes {
    dash_frame_max: 21,
    dash_frame_min: 11,
    dash_initial_velocity: 2.02,
    dash_acceleration_a: 0.1,
    dash_acceleration_b: 0.02,
    dash_max_velocity: 2.2,
    walk_initial_velocity: 0.16,
    walk_acceleration: 0.2,
    walk_max_velocity: 1.6,
    traction: 0.08,
    runturn_breakpoint: 16,
    jumpsquat: 3,
    jump_initial_y_velocity: 3.68,
    hop_initial_y_velocity: 2.1,
    jump_initial_x_velocity: 0.72,
    jump_max_horizontal_velocity: 1.7,
    ground_air_momentum_conservation: 0.83,
    doublejmup_momentum_conservation: 0.9,
    gravity: 0.23,
    aerial_max_horizontal_velocity: 0.83,
    air_friction: 0.02,
    terminal_velocity: 2.8,
    fastfall_velocity: 3.4,
    air_mobility_a: 0.06,
    air_mobility_b: 0.02,
    doublejump_multiplier: 1.2,
};