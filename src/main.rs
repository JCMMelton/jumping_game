extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::math::*;

use std::f64;
use rand::Rng;

mod constants;
mod util;
mod game;
mod player;
mod platform;
mod background;

use constants::*;
use game::*;
use player::*;

fn main() {

    let opengl: OpenGL = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Jumping Game",
        [WIDTH as u32, HEIGHT as u32]
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(opengl);

    game.reset();


    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if game.alive {
            if let Some(m) = e.mouse_cursor_args() {
                game.on_mouse_mov(&m);
            }

            if let Some(u) = e.update_args() {
                game.update(&u);
            }

            if let Some(k) = e.release_args() {
                match k {
                    Button::Keyboard(key) => game.on_key_release(key),
                    _ => {}
                }
            }
        }

        if let Some(k) = e.press_args() {
            match k {
                Button::Keyboard(key) => game.on_key_press(key),
                _ => {}
            }
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(e) = e.resize_args() {
            game.on_resize(&e);
        }

    }
}
