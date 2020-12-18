pub mod helper;
pub mod characters;
mod FOX;
use crate::player::Player;
use crate::input::Input;

#[derive(Debug)]
pub struct Character {
    pub ACTIONSTATE: &'static CharacterActionStates,
    pub ATTRIBUTE: &'static CharacterAttributes,
    pub FRAMEDATA: &'static CharacterFrameData, 
}


#[derive(Debug)]
pub struct CharacterActionStates {
    pub WAIT: &'static dyn ActionState,
    pub KNEEBEND: &'static dyn ActionState,
    pub JUMPF: &'static dyn ActionState,
    pub FALL: &'static dyn ActionState,
}


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

// #[derive(Debug)]
// pub enum CharacterID {
//     FOX = 10,
// }

#[derive(Debug)]
pub struct CharacterFrameData {
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


pub trait ActionState : std::fmt::Debug + Sync {
    fn init(&'static self, player: &mut Player, input: &Input);
    fn main(&'static self, player: &mut Player, input: &Input);
    fn interrupt(&'static self, player: &mut Player, input: &Input) -> bool;
}



// #[allow(unreachable_patterns)]
// pub fn frame_data(character_id: &CharacterID) -> &'static FrameData {
//     match character_id {
//         CharacterID::FOX => &FOXFRAMEDATA,
//         _ => &FOXFRAMEDATA,
//     }
// }