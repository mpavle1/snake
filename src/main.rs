use std::thread::sleep;

mod snake;
mod common;

fn main() {
    let food = common::Position {
        x: 5,
        y: 5
    };

    let mut snake =  snake::Snake::new();

    loop {
        snake.update();

        for i in 0..=common::HEIGHT {
            for j in 0..=common::WIDTH {
                if i == 0 {
                    if j == common::WIDTH {
                        println!("{}", "-");
                    } else {
                        print!("{}", "-");
                    }
                } else if i == common::HEIGHT {
                    if j == common::WIDTH {
                        println!("{}", "-");
                    } else {
                        print!("{}", "-");
                    }
                } else if j == 0 {
                    print!("{}", "|");
                } else if j == common::WIDTH {
                    println!("{}", "|");
                } else {
                    if snake.render(common::Position { x: j, y: i }) {
                        continue;
                    }

                    if food.x == j && food.y == i {
                        print!("{}", "*");
                        continue;
                    }

                    print!("{}", " ");
                }
            }
        }

        sleep(common::SLEEP_DURATION);
        
        print!("\x1B[2J\x1B[1;1H");
    }
}
