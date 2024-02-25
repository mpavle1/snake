use std::collections::LinkedList;

use piston_window::{types::Color, *};

use crate::common::{Directions, Position, BACKGROUND, BLOCK_SIZE, HEIGHT, SNAKE, WIDTH};

pub struct Snake {
    pub direction: Directions,
    pub head: Position,
    pub body: LinkedList<Position>,
}

pub fn blocks_in_pixels(n: u32) -> u32 {
    n * BLOCK_SIZE as u32
}

pub fn draw_block(ctx: &Context, g: &mut G2d, c: Color, pos: &Position) {
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

pub fn draw_snake_head(ctx: &Context, g: &mut G2d, c: Color, pos: &Position, dir: &Directions) {
    draw_block(ctx, g, c, pos);

    fn draw_eye(ctx: &Context, g: &mut G2d, x: f64, y: f64) {
        rectangle(BACKGROUND, [x, y, 5.0, 5.0], ctx.transform, g);
    }

    let (x, y) = (
        blocks_in_pixels(pos.x) as f64,
        blocks_in_pixels(pos.y) as f64,
    );

    let block = blocks_in_pixels(1) as f64;

    match dir {
        Directions::UP => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
        }
        Directions::RIGHT => {
            draw_eye(ctx, g, x + block - 10.0, y + 5.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Directions::DOWN => {
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
            draw_eye(ctx, g, x + block - 10.0, y + block - 10.0);
        }
        Directions::LEFT => {
            draw_eye(ctx, g, x + 5.0, y + 5.0);
            draw_eye(ctx, g, x + 5.0, y + block - 10.0);
        }
    }
}

impl Snake {
    pub fn update(&mut self) {
        self.body.push_front(self.head);
        self.body.pop_back();

        match self.direction {
            Directions::UP => {
                self.head.y -= 1;

                if self.head.y == 0 {
                    self.head.y = HEIGHT - 1;
                }
            }
            Directions::DOWN => {
                self.head.y += 1;

                if self.head.y == HEIGHT {
                    self.head.y = 1;
                }
            }
            Directions::LEFT => {
                self.head.x -= 1;

                if self.head.x == 0 {
                    self.head.x = WIDTH - 1;
                }
            }
            Directions::RIGHT => {
                self.head.x += 1;
                if self.head.x == WIDTH {
                    self.head.x = 1;
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            head: Position {
                x: (WIDTH as f32 / 2.0).floor() as u32,
                y: (HEIGHT as f32 / 2.0).floor() as u32,
            },
            body: LinkedList::from([Position {
                x: (WIDTH as f32 / 2.0).floor() as u32,
                y: ((WIDTH as f32 / 2.0).floor() - 1.0) as u32,
            }]),
            direction: Directions::DOWN,
        }
    }

    pub fn update_direction(&mut self, key: keyboard::Key) {
        match key {
            Key::A | Key::Left => self.direction = Directions::LEFT,
            Key::W | Key::Up => self.direction = Directions::UP,
            Key::D | Key::Right => self.direction = Directions::RIGHT,
            Key::S | Key::Down => self.direction = Directions::DOWN,
            _ => {}
        }
    }

    pub fn grow(&mut self) {
        let last_pos = match self.body.clone().into_iter().last() {
            Some(pos) => pos.clone(),
            None => self.head.clone(),
        };
        self.body.push_back(last_pos);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in self.body.iter() {
            draw_block(ctx, g, SNAKE, block)
        }

        draw_snake_head(ctx, g, SNAKE, &self.head, &self.direction);
    }
}
