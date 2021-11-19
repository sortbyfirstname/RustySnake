use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BRICK_SIZE: f64 = 25.0;

pub fn to(game_coord: i32) -> f64 {
    (game_coord as f64) * BRICK_SIZE
}

pub fn to_u32(game_coord: i32) -> u32 {
    to(game_coord) as u32
}

pub fn draw_brick(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to(x);
    let gui_y = to(y);

    rectangle(
        color,
        [gui_x, gui_y, BRICK_SIZE, BRICK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d) {
    let x = to(x);
    let y = to(y);

    rectangle(
        color,
        [x, y, BRICK_SIZE * (width as f64), BRICK_SIZE * (height as f64)],
        con.transform,
        g
    );
}