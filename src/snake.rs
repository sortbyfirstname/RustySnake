use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw::draw_brick;

const SNAKE_COLOR: Color = [0.70, 0.20, 0.80, 1.00];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    x: i32,
    y: i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Brick>,
    tail: Option<Brick>
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Brick> = LinkedList::new();
        body.push_back(Brick {
            x: x + 2,
            y
        });
        body.push_back(Brick {
            x: x + 1,
            y
        });
        body.push_back(Brick {
            x,
            y
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for brick in &self.body {
            draw_brick(SNAKE_COLOR, brick.x, brick.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_brick = self.body.front().unwrap();
        (head_brick.x, head_brick.y)
    }

    pub fn forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(dir) => self.direction = dir,
            None => ()
        }

        let (current_x, current_y): (i32, i32) = self.head_position();

        let new_brick = match self.direction {
            Direction::Up => Brick {
                x: current_x,
                y: current_y - 1
            },
            Direction::Down => Brick {
                x: current_x,
                y: current_y + 1
            },
            Direction::Left => Brick {
                x: current_x - 1,
                y: current_y
            },
            Direction::Right => Brick{
                x: current_x + 1,
                y: current_y
            }
        };

        self.body.push_front(new_brick);
        let removed_brick = self.body.pop_back().unwrap();
        self.tail = Some(removed_brick);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(dir) => moving_dir = dir,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y)
        }
    }

    pub fn add_tail(&mut self) {
        let brick = self.tail.clone().unwrap();
        self.body.push_back(brick);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;

        for brick in &self.body {
            if x == brick.x && y == brick.y {
                return true;
            }

            ch += 1;

            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;

    }
}