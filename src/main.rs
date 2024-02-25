use core::time::Duration;

use piston_window::*;
use piston_window::{PistonWindow, WindowSettings};

pub const SLEEP_DURATION: Duration = Duration::from_millis(500);

mod common;
mod game;
mod snake;

fn main() {
    let size = [
        common::BLOCK_SIZE as u32 * common::WIDTH,
        common::BLOCK_SIZE as u32 * common::HEIGHT,
    ];
    let mut game = game::Game::new();
    let mut window: PistonWindow = WindowSettings::new("Snake", size)
        .resizable(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.update_snake_direction(key);
        }

        window.draw_2d(&event, |ctx, g, _| {
            clear(common::BACKGROUND, g);
            game.draw(ctx, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
