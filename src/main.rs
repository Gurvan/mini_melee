mod input;
use input::{Input, InputManager};

fn main() {
    println!("Hello, world!");

    let mut input_manager = InputManager::new().unwrap();
    let mut input0 = Input::new();

    loop {
        input_manager.step(&mut input0, 1);
        // println!("{:?} / {:?} | {:?} / {:?} | {:?} / {:?}", input0.current.stick_x, input0.current.stick_y, input0.current.c_x, input0.current.c_y, input0.current.l_analog, input0.current.r_analog);
        println!("{:?}", input0.current);
    }
}
