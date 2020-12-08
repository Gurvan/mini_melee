#[derive(Debug)]
pub struct CharacterAttributes {
    pub dash_frame_max: i32,
    pub dash_frame_min: i32,
    pub dash_initial_velocity: f32,
    pub dash_acceleration_a: f32,
    pub dash_acceleration_b: f32,
    pub dash_max_velocity: f32,
    pub walk_initial_velocity: f32,
    pub walk_acceleration: f32,
    pub walk_max_velocity: f32,
    pub traction: f32,
    pub runturn_breakpoint: i32,
    pub jumpsquat: i32,
    pub jump_initial_y_velocity: f32,
    pub hop_initial_y_velocity: f32,
    pub jump_initial_x_velocity: f32,
    pub jump_max_horizontal_velocity: f32,
    pub ground_air_momentum_conservation: f32,
    pub doublejmup_momentum_conservation: f32,
    pub gravity: f32,
    pub aerial_max_horizontal_velocity: f32,
    pub air_friction: f32,
    pub terminal_velocity: f32,
    pub fastfall_velocity: f32,
    pub air_mobility_a: f32,
    pub air_mobility_b: f32,
    pub doublejump_multiplier: f32,
}

impl CharacterAttributes {
    pub fn new() -> CharacterAttributes {
        CharacterAttributes {  // fox
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
        }
    }
}

#[derive(Debug)]
pub enum CharacterID {
    FOX = 10,
}

#[derive(Debug)]
pub struct FrameData {
    pub WAIT: i32,
    pub DASH: i32,
    pub RUN: i32,
    pub RUNBRAKE: i32,
    pub RUNTURN: i32,
    pub WALK: i32,
    pub JUMPF: i32,
    pub JUMPB: i32,
    pub FALL: i32,
    pub FALLAERIAL: i32,
    pub FALLSPECIAL: i32,
    pub JUMPAERIALF: i32,
    pub JUMPAERIALB: i32,
}

pub static FOXFRAMEDATA: FrameData = FrameData {
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

#[allow(unreachable_patterns)]
pub fn frame_data(character_id: &CharacterID) -> &'static FrameData {
    match character_id {
        CharacterID::FOX => &FOXFRAMEDATA,
        _ => &FOXFRAMEDATA,
    }
}