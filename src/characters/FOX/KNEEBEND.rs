use crate::player::Player;
use crate::input::Input;
use crate::characters::ActionState;
// use crate::render::animations;
// use super::JUMP::JUMP;

pub struct Kneebend {
    // animation: &'static [&'static [(f32, f32)]],
}

impl Kneebend {
    const fn new() -> Kneebend {
        Kneebend {
            // animation: &animations::KNEEBEND::KNEEBEND,
        }
    }
}

pub static KNEEBEND: &Kneebend = &Kneebend::new();

impl ActionState for Kneebend {
    // fn animation(&self) -> &'static [&'static [(f32, f32)]] {
    //     return self.animation;
    // }
    fn init(&'static self, player: &mut Player, input: &Input) {
        player.action_state = player.character.ACTIONSTATE.KNEEBEND;
        player.timer = 0;
        player.physic.jump_type = 1;
        self.main(player, input);
    }
    fn main(&'static self, player: &mut Player, input: &Input) {
        player.timer += 1;
        // Define shorthop / full jump. To do later
        if !self.interrupt(player, input) {
             if !input.current.x {
                 player.physic.jump_type = 0;
             }
        }
    }
    fn interrupt(&'static self, player: &mut Player, input: &Input) -> bool {
        if player.timer == player.character.ATTRIBUTE.jumpsquat {
            player.physic.position.y += 0.001;
        }
        if player.timer > player.character.ATTRIBUTE.jumpsquat {
            player.character.ACTIONSTATE.JUMPF.init(player, input);
            return true;
        }
        return false;
    }
}

impl std::fmt::Debug for Kneebend {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Kneebend")
    }
}