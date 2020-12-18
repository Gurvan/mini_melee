use crate::player::Player;
use crate::input::Input;
// use crate::framedata::frame_data;
// use crate::render::animations;
use crate::characters::ActionState;
// use super::FALL::FALL;
// use super::JUMPAERIAL::JUMPAERIAL;
use crate::characters::helper;

pub struct Jumpf {
    // animation: &'static [&'static [(f32, f32)]],
}

impl Jumpf {
    const fn new() -> Jumpf {
        Jumpf {
            // animation: animations::JUMP::JUMP,
        }
    }
}

pub static JUMPF: &Jumpf = &Jumpf::new();

impl ActionState for Jumpf {
    // fn animation(&'static self) -> &'static [&'static [(f32, f32)]] {
    //     return self.animation;
    // }
    fn init(&'static self, player: &mut Player, input: &Input) {
        player.action_state = JUMPF;
        player.timer = 0;
        if player.physic.jump_type > 0 {
            player.physic.c_vel.y += player.character.ATTRIBUTE.jump_initial_y_velocity;
        } else {
            player.physic.c_vel.y += player.character.ATTRIBUTE.hop_initial_y_velocity;
        }
        // Momentum conservation
        let lsx: f32 = input.current.stick_x;
        player.physic.c_vel.x = player.physic.c_vel.x * player.character.ATTRIBUTE.ground_air_momentum_conservation
                              + lsx * player.character.ATTRIBUTE.jump_initial_x_velocity;
        if player.physic.c_vel.x.abs() > player.character.ATTRIBUTE.jump_max_horizontal_velocity {
            player.physic.c_vel.x = player.character.ATTRIBUTE.jump_max_horizontal_velocity * player.physic.c_vel.x.signum();
        }
    
        player.physic.grounded = false;
        self.main(player, input);
    }
    fn main(&'static self, player: &mut Player, input: &Input) {
        player.timer += 1;
        if !self.interrupt(player, input) {
            helper::fall(player, input);
            helper::aerial_drift(player, input);
        }
    }
    fn interrupt(&'static self, player: &mut Player, input: &Input) -> bool {
        if helper::land(player, input) {
            return true;
        }
        // if helper::check_doublejump(player, input) {
        //     JUMPAERIAL.init(player, input);
        //     return true;
        // }
        // if player.timer > player.character.FRAMEDATA.JUMPF {
        //     player.character.ACTIONSTATE.FALL.init(player, input);
        //     return true;
        // }
        return false;
    }
}

// impl std::fmt::Debug for Jump {
//     fn fmt(&'static self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(fmt, "JUMP")
//     }
// }

impl std::fmt::Debug for Jumpf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Jump")
    }
}