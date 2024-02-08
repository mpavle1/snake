use crate::common::{self, Position};
use crate::snake::Snake;

struct Food {
    position: Position
}

impl Food {
    fn render(&self, pos: Position) -> bool {
        let (x,y) = (pos.x, pos.y);

        if self.position.x == x && self.position.y == y {
            print!("{}", "*");
            return true;
        }

        return false;
    }

    fn new() -> Self {
        return  Self {
            position: Position {
                x: 5,
                y: 5
            }
        };
    }
}

pub struct Game {
    food: Food,
    snake: Snake,
    is_over: bool
}

impl Game {
    pub fn update(&mut self) {
        self.snake.update(self.food.position);
    }

    pub fn render(&self){
        for i in 0..=common::HEIGHT {
            for j in 0..=common::WIDTH {
                if i == 0 {
                    if j == common::WIDTH {
                        println!("{}", "-");
                    } else {
                        print!("{}", "-");
                    }
                    continue;
                }
                if i == common::HEIGHT {
                    if j == common::WIDTH {
                        println!("{}", "-");
                    } else {
                        print!("{}", "-");
                    }
                    continue;
                }
                if j == 0 {
                    print!("{}", "|");
                    continue;
                }
                if j == common::WIDTH {
                    println!("{}", "|");
                    continue;
                }
                
                let current_position = Position { x: j, y: i };

                if self.snake.render(current_position) {
                    continue;
                }
                if self.food.render(current_position) {
                    continue;
                }

                print!("{}", " ");
            }
        }
    }

    pub fn new() -> Self {
        return Self {
            snake: Snake::new(),
            food: Food::new(),
            is_over: false
        };
    }
}