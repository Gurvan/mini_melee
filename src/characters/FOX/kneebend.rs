use crate::player::Player;
use crate::input::Input;
use crate::characters::ActionState;
use super::ATTRIBUTE;
use super::ACTIONSTATE;

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

pub const KNEEBEND: &Kneebend = &Kneebend::new();

impl ActionState for Kneebend {
    // fn animation(&self) -> &'static [&'static [(f32, f32)]] {
    //     return self.animation;
    // }
    fn init(&'static self, player: &mut Player, input: &Input) {
        player.action_state = self;
        player.timer = 0;
        player.physic.jump_type = 1;
        self.main(player, input);
    }
    fn main(&'static self, player: &mut Player, input: &Input) {
        player.timer += 1;
        // Define shorthop / full jump. To do later
        if !self.interrupt(player, input) {
             if !input.current().y { // X always short hop, TODO: put it back
            //  if !input.current().x && !input.current().y {
                 player.physic.jump_type = 0;
             }
        }
    }
    fn interrupt(&'static self, player: &mut Player, input: &Input) -> bool {
        if player.timer == ATTRIBUTE.jumpsquat {
            player.physic.position.y += 0.001;
        }
        if player.timer > ATTRIBUTE.jumpsquat {
            if input.previous2().stick_x * player.physic.facing as f32 >= -0.3 {
                ACTIONSTATE.JUMPF.init(player, input);
            } else {
                ACTIONSTATE.JUMPB.init(player, input);
            }
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