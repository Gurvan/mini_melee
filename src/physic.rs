#[derive(Debug)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug)]
pub struct Physic {
    pub position: Vec2D<f32>,
    pub facing: i8,
    pub grounded: bool,
    pub fastfall: bool,
    pub doublejump: bool,
    pub dash_buffer: bool,
    pub jump_type: i8,
    pub c_vel: Vec2D<f32>,  // character velocity, from movements
}

impl Physic {
    pub fn new() -> Physic {
        Physic {
            position: Vec2D {x: 0.0, y: 0.0},
            facing: 1,  // left: -1, right: 1
            grounded: true,
            fastfall: false,
            doublejump: false,
            dash_buffer: false,
            jump_type: 0,
            c_vel: Vec2D {x: 0.0, y: 0.0},
        }
    }
}

pub fn process(physic: &mut Physic) {
    physic.position.x += physic.c_vel.x;
    physic.position.y += physic.c_vel.y;
}