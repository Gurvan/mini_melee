use crate::player::Player;
use crate::input::Input;
use crate::characters::ActionState;
use crate::characters::helper;
use super::FRAMEDATA;
use super::ACTIONSTATE;

pub struct Wait {
    frame_data: i32,
    // animation: &'static [&'static [(f32, f32)]],
}

impl Wait {
    const fn new() -> Wait {
        Wait {
            frame_data: FRAMEDATA.WAIT,
            // animation: animations::WAIT::WAIT,
        }
    }
}

pub const WAIT: &'static Wait = &Wait::new();

impl ActionState for Wait {
    // fn animation(&self) -> &'static [&'static [(f32, f32)]] {
    //     return self.animation;
    // }
    fn init(&'static self, player: &mut Player, input: &Input) {
        player.action_state = ACTIONSTATE.WAIT;
        player.timer = 0;
        self.main(player, input);
    }
    fn main(&'static self, player: &mut Player, input: &Input) {
        player.timer += 1;
        if !self.interrupt(player, input) {
            helper::traction(player, false);
        }
    }
    fn interrupt(&'static self, player: &mut Player, input: &Input) -> bool {
        if helper::check_jump(player, input) {
            ACTIONSTATE.KNEEBEND.init(player, input);
            return true;
        }
        // if helper::check_dash(player, input) {
        //     DASH.init(player, input);
        //     return true;
        // }
        // if helper::check_smashturn(player, input) {
        //     SMASHTURN.init(player, input);
        //     return true;
        // }
        // if helper::check_tiltturn(player, input) {
        //     helper::buffer_dash(player, input);
        //     TILTTURN.init(player, input);
        //     return true;
        // }
        // if input.current.stick_x.abs() > 0.0 {
        //     WALK.init(player, input);
        //     return true;
        // }
        if player.timer >= self.frame_data {
            self.init(player, input);
            return true;
        }
        return false;
    }
}

impl std::fmt::Debug for Wait {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Wait")
    }
}