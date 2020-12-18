use std::time::{Duration, Instant};
mod input;
mod player;
mod characters;
mod physic;

pub use input::{Input, InputManager};
pub use player::Player;


fn main() {
    println!("Hello, world!");

    let mut input_manager = InputManager::new().unwrap();
    let mut input0 = Input::new();
    let p0 = &mut Player::new();

    println!("{:?}", p0);

    let one_frame = Duration::from_micros(16667);
    let mut t = Instant::now();
    loop {
        if t.elapsed() > one_frame {
            // println!("{:?}", 1. / t.elapsed().as_secs_f64());
            t = Instant::now();
            input_manager.step(&mut input0, 1);
            p0.action_state.main(p0, &input0);
            // println!("{:?} / {:?} | {:?} / {:?} | {:?} / {:?}", input0.current.stick_x, input0.current.stick_y, input0.current.c_x, input0.current.c_y, input0.current.l_analog, input0.current.r_analog);
            println!("{:?}", input0.current());
            println!("{:?}", p0.action_state);
        }

    }
}
