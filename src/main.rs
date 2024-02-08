use std::thread::sleep;
use core::time::Duration;

pub const SLEEP_DURATION: Duration  = Duration::from_millis(500);

mod game;
mod snake;
mod common;

fn main() {
    let mut game = game::Game::new();

    loop {
        game.update();
        game.render();

        sleep(SLEEP_DURATION);
        
        print!("\x1B[2J\x1B[1;1H");
    }
}
