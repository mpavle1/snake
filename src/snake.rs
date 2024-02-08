use std::collections::LinkedList;

use crate::common::{ Directions, HEIGHT, WIDTH, Position };

pub struct Snake {
    pub direction: Directions,
    pub head: Position,
    pub body: LinkedList<Position>
}

impl Snake {
    pub fn update(&mut self, food_position: Position) {
        self.body.push_front(self.head);
        self.body.pop_back();

        // ako su im iste X ose a razlitice Y, treba da se rotira glava ka gore ili dole
        // ako su im iste Y ose a razlitice X, treba da se rotira glava ka levo ili desno
        // ako je oboje isto, treba odluciti sa jednim od 2 (x/Y)

        // ubaciti i da prati gde je njegovo telo u jednom momentu

        match self.direction {
            Directions::UP => {
                self.head.y -= 1;

                if self.head.y == 0 {
                    self.head.y = HEIGHT - 1;
                }
            },
            Directions::DOWN => {
                self.head.y += 1;

                if self.head.y == HEIGHT {
                    self.head.y = 1;
                }
            },
            Directions::LEFT => {
                self.head.x -= 1;

                if self.head.x == 0 {
                    self.head.x = WIDTH - 1;
                }
            },
            Directions::RIGHT => {
                self.head.x += 1;
                if self.head.x == WIDTH {
                    self.head.x = 1;
                }
            },
        }
    }

    pub fn render(&self, pos: Position) -> bool {
        let (x,y) = (pos.x, pos.y);
        let is_head_position = self.head.x == x && self.head.y == y;
        let is_body_position = self.body.iter().any(|pos| pos.x == x && pos.y == y);

        if is_head_position {
            match self.direction {
                Directions::UP => print!("{}", "^"),
                Directions::DOWN => print!("{}", "V"),
                Directions::LEFT => print!("{}", "<"),
                Directions::RIGHT => print!("{}", ">"),
            }

            return true;
        }

        if is_body_position {
            print!("{}", "0");
            return true;
        }

        return false;
    }

    pub fn new() -> Self{
        return Self {
            head: Position {
                x: (WIDTH as f32 / 2.0).floor() as u32,
                y: (HEIGHT as f32 / 2.0).floor() as u32
            },
            body: LinkedList::from(
                [
                    Position {
                        x: (WIDTH as f32 / 2.0).floor() as u32,
                        y: ((WIDTH as f32 / 2.0).floor() - 1.0) as u32
                    }
                ]
            ),
            direction: Directions::DOWN
        };
    }
}