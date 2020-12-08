use crate::physic::Physic;
use crate::characters::{CharacterID, CharacterAttributes};


#[derive(Debug)]
pub struct Player {
    pub physic: Physic,
    pub timer: i32,
    pub residual_timer: f32,
    pub character: CharacterID,
    pub character_attributes: CharacterAttributes,
    // pub action_state: &'static dyn ActionState,
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            physic: Physic::new(),
            timer: 0,
            residual_timer: 0.0,
            character: CharacterID::FOX,
            character_attributes: CharacterAttributes::new(),
            // action_state: &WAIT,
        };
        return player;
    } 
}