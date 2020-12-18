use crate::player::Player;
use crate::input::Input;
// use super::ActionState;

pub fn fall(player: &mut Player, input: &Input) {
    if !player.physic.fastfall {
        player.physic.c_vel.y -= player.character.ATTRIBUTE.gravity;
        if player.physic.c_vel.y < -player.character.ATTRIBUTE.terminal_velocity {
            player.physic.c_vel.y = -player.character.ATTRIBUTE.terminal_velocity;
        }
        // if input.down && !input.previous_down && player.physic.c_vel.y < 0.0 {
        if input.current.stick_y < -0.65 && player.physic.c_vel.y < 0.0 {
            player.physic.c_vel.y = -player.character.ATTRIBUTE.fastfall_velocity;
            player.physic.fastfall = true;
        }
    }
}

pub fn aerial_drift(player: &mut Player, input: &Input) {
    let air_velocity_bound: f32;
    let lsx = input.current.stick_x;
    if lsx.abs() < 0.3 {
        air_velocity_bound = 0.0;
    } else {
        air_velocity_bound = lsx * player.character.ATTRIBUTE.aerial_max_horizontal_velocity;
    }
    // if input.left {
    //     air_velocity_bound = -player.character.ATTRIBUTE.aerial_max_horizontal_velocity;
    // } else if input.right {
    //     air_velocity_bound = player.character.ATTRIBUTE.aerial_max_horizontal_velocity;
    // } else {
    //     air_velocity_bound = 0.0;
    // }
    if player.physic.c_vel.x.abs() > air_velocity_bound.abs() {
        if player.physic.c_vel.x > 0.0 {
            player.physic.c_vel.x -= player.character.ATTRIBUTE.air_friction;
            if player.physic.c_vel.x < 0.0 {
                player.physic.c_vel.x = 0.0;
            }
        } else {
            player.physic.c_vel.x += player.character.ATTRIBUTE.air_friction;
            if player.physic.c_vel.x > 0.0 {
                player.physic.c_vel.x = 0.0;
            }
        }
    // } else if (input.left || input.right) && (player.physic.c_vel.x.abs() < air_velocity_bound.abs()) {
    //     if input.left {
    //         player.physic.c_vel.x -= player.character.ATTRIBUTE.air_mobility_a + player.character.ATTRIBUTE.air_mobility_b;
    //     } else if input.right {
    //         player.physic.c_vel.x += player.character.ATTRIBUTE.air_mobility_a + player.character.ATTRIBUTE.air_mobility_b;
    //     }
    // }
    } else if lsx.abs() > 0.3 && (player.physic.c_vel.x.abs() < air_velocity_bound.abs()) {
        player.physic.c_vel.x += lsx * player.character.ATTRIBUTE.air_mobility_a + lsx.signum() * player.character.ATTRIBUTE.air_mobility_b;

    }

    // if !input.left && !input.right {
    if lsx < 0.3 {
        if player.physic.c_vel.x > 0.0 {
            player.physic.c_vel.x -= player.character.ATTRIBUTE.air_friction;
            if player.physic.c_vel.x < 0.0 {
                player.physic.c_vel.x = 0.0;
            }
        } else {
            player.physic.c_vel.x += player.character.ATTRIBUTE.air_friction;
            if player.physic.c_vel.x > 0.0 {
                player.physic.c_vel.x = 0.0;
            }
        }
    }
}


pub fn land(player: &mut Player, input: &Input) -> bool {
    if player.physic.position.y < 0.0 {
        player.physic.position.y = 0.0;
        player.physic.c_vel.y = 0.0;
        player.physic.fastfall = false;
        player.physic.doublejump = false;
        player.character.ACTIONSTATE.WAIT.init(player, input);
        return true;
    }
    return false;
}

pub fn traction(player: &mut Player, double_traction: bool) {
    if player.physic.c_vel.x > 0.0 {
        if double_traction && player.physic.c_vel.x > player.character.ATTRIBUTE.walk_max_velocity {
            player.physic.c_vel.x -= player.character.ATTRIBUTE.traction * 2.0;
        } else {
            player.physic.c_vel.x -= player.character.ATTRIBUTE.traction;
        }
        if player.physic.c_vel.x < 0.0 {
            player.physic.c_vel.x = 0.0;
        }
    } else {
        if double_traction && player.physic.c_vel.x < -player.character.ATTRIBUTE.walk_max_velocity {
            player.physic.c_vel.x += player.character.ATTRIBUTE.traction * 2.0;
        } else {
            player.physic.c_vel.x += player.character.ATTRIBUTE.traction;
        }
        if player.physic.c_vel.x > 0.0 {
            player.physic.c_vel.x = 0.0;
        }
    }
}

#[allow(unused_variables)]
pub fn check_jump(player: &mut Player, input: &Input) -> bool {
    return (input.current.x && !input.previous.x) || (input.current.y && !input.previous.y);
}

pub fn check_dash(player: &mut Player, input: &Input) -> bool {
    return (input.current.stick_x * player.physic.facing as f32) > 0.79 && (input.previous2.stick_x * player.physic.facing as f32) < 0.3;
}

pub fn check_smashturn(player: &mut Player, input: &Input) -> bool {
    return (input.current.stick_x * player.physic.facing as f32) < -0.79 && (input.previous2.stick_x * player.physic.facing as f32) > -0.3;
}

pub fn check_tiltturn(player: &mut Player, input: &Input) -> bool {
    return (input.current.stick_x * player.physic.facing as f32) < -0.3;
}

pub fn buffer_dash(player: &mut Player, input: &Input) {
    player.physic.dash_buffer = input.previous.stick_x * player.physic.facing as f32 > -0.3;
}

pub fn check_foxtrot(player: &mut Player, input: &Input) -> bool {
    return check_dash(player, input);
}

pub fn check_doublejump(player: &mut Player, input: &Input) -> bool {
    // return (input.x & !input.previous_x & !player.physic.doublejump);
    return input.current.x & !input.previous.x & !player.physic.doublejump;
}

