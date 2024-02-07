use core::time::Duration;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT
}


pub const WIDTH: u32 = 20;
pub const HEIGHT: u32 = 20;

pub const SLEEP_DURATION: Duration  = Duration::from_millis(500);