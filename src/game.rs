use crate::snake;
use crate::common;

struct Game {
    food: Position,
    snake: Snake,
    is_over: bool
}