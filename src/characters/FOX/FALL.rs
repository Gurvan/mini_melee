use crate::player::Player;
use crate::input::Input;
use crate::characters::ActionState;
use crate::characters::helper;
// use super::JUMPAERIAL::JUMPAERIAL;

pub struct Fall {  
    // animation: &'static [&'static [(f32, f32)]],
}

impl Fall {
    const fn new() -> Fall {
        Fall {
            // animation: animations::FALL::FALL,
        }
    }
}

pub static FALL: &Fall = &Fall::new();

impl ActionState for Fall {
    // fn animation(&'static self) -> &'static [&'static [(f32, f32)]] {
    //     return self.animation;
    // }
    fn init(&'static self, player: &mut Player, input: &Input) {
        player.action_state = FALL;
        player.timer = 0;
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
        //     player.character.ACTIONSTATE.JUMPAERIAL.init(player, input);
        //     return true;
        // }
        if player.timer > player.character.FRAMEDATA.FALL {
            FALL.init(player, input);
            return true;
        }
        return false;
    }
}

impl std::fmt::Debug for Fall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Fall")
    }
}