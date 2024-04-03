extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BG_COLOR: Color = [0.6, 0.6, 0.6, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) =e.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&e, |c, g, _| {
            clear(BG_COLOR, g);
            game.draw(&c, g);
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
