use crate::common::{self, Position, BLOCK_SIZE, HEIGHT, WIDTH};
use crate::snake::Snake;
use piston_window::{keyboard, rectangle, Context, G2d};
use rand::{self, Rng};

fn fps_in_ms() -> f64 {
    1.0 / 6.0
}

fn get_random_positon() -> Position {
    let mut random = rand::thread_rng();

    Position {
        x: random.gen_range(0..WIDTH) as u32,
        y: random.gen_range(0..HEIGHT) as u32,
    }
}

pub fn draw_block(ctx: &Context, g: &mut G2d, c: [f32; 4], pos: &Position) {
    rectangle(
        c,
        [
            pos.x as f64 * BLOCK_SIZE,
            pos.y as f64 * BLOCK_SIZE,
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        ctx.transform,
        g,
    );
}

struct Food {
    position: Position,
}

impl Food {
    fn new() -> Self {
        Self {
            position: Position { x: 5, y: 5 },
        }
    }
}

pub struct Game {
    food: Food,
    snake: Snake,
    waiting_time: f64,
}

impl Game {
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > fps_in_ms() {
            self.waiting_time = 0.0;

            self.snake.update();

            if self.snake.head == self.food.position {
                self.food.position = get_random_positon();
                self.snake.grow();

                self.snake.update();
            }
        }
    }

    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            food: Food::new(),
            waiting_time: 0.0,
        }
    }

    pub fn update_snake_direction(&mut self, key: keyboard::Key) {
        self.snake.update_direction(key);
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        draw_block(&ctx, g, common::FRUIT, &self.food.position);
        self.snake.draw(&ctx, g)
    }
}
