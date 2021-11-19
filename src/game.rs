use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};
use crate::snake::{Direction, Snake};
use crate::draw::{draw_brick, draw_rectangle};

const FOOD_COLOR: Color = [0.00, 0.80, 0.00, 1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const GAME_OVER_COLOR: Color = [0.95, 0.00, 0.00, 0.40];

const FPS: f64 = 0.1;
const RESTART: f64 = 2.0;

pub struct Game {
    snake: Snake,
    food: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    wait: f64,
    score: i32
}

impl Game{
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(3, 2),
            food: true,
            food_x: 5,
            food_y: 7,
            width,
            height,
            game_over: false,
            wait: 0.10,
            score: 0 // TODO: Add score tracking and updating
        }
    }

    pub fn key(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),

            Key::W => Some(Direction::Up),
            Key::A => Some(Direction::Left),
            Key::S => Some(Direction::Down),
            Key::D => Some(Direction::Right),

            _ => None
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food {
            draw_brick(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw borders
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.wait += delta_time;

        if self.game_over {
            if self.wait > RESTART {
                self.restart();
            }
            return;
        }

        if !self.food {
            self.add_food();
        }

        if self.wait > FPS {
            self.update_snake(None);
        }
    }

    fn check_eat(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food && self.food_x == head_x && self.food_y == head_y {
            self.food = false;
            self.snake.add_tail();
            self.score += 1;
        }
    }

    fn check_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0
        && next_x < self.width - 1
        && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height -1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_alive(dir) {
            self.snake.forward(dir);
            self.check_eat();
        } else {
            self.game_over = true;
        }
        self.wait = 0.00;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(3, 2);
        self.food = true;
        self.food_x = thread_rng().gen_range(1..self.width - 1);
        self.food_y = thread_rng().gen_range(1..self.height - 1);
        self.wait = 0.00;
        self.game_over = false;
        self.score = 0;
    }
}