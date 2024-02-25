use piston_window::types::Color;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub const WIDTH: u32 = 20;
pub const HEIGHT: u32 = 20;
pub const BLOCK_SIZE: f64 = 25.0;

pub const BACKGROUND: Color = [0.0, 0.0, 0.0, 1.0];
pub const SNAKE: Color = [0.1, 0.9, 0.1, 1.0];
pub const FRUIT: Color = [1.0, 0.0, 0.0, 1.0];
