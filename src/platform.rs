extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate nalgebra;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use self::nalgebra::core::{ Vector2 };

use std::f64;
use rand::Rng;

use constants::*;
use util::*;

pub struct Platform {
    pub id: u32,
    pub width: f64,
    pub height: f64,
    color: [f32; 4],
    pub pos: Vector2<f64>,
    pub rect: [f64; 4],

}

impl Platform {

    pub fn new(id: u32) -> Self {
        let mut rng = rand::thread_rng();
        let rx = (rng.gen::<f64>() * WIDTH) - ORIGIN[0];
        let rw = (rng.gen::<f64>() * 230.0) + 20.0;
        let pos = Vector2::new(rx, -ORIGIN[1]-20.0);
        Platform {
            id,
            width: rw,
            height: 10.0,
            color: [0.5, 0.5, 0.5, 1.0],
            pos,
            rect: [pos.x - rw/2.0, pos.y - 5.0, rw, 10.0],
        }
    }

    pub fn new_at_location(x: f64, y: f64, id: u32) -> Self {
        let pos = Vector2::new(x, y);
        Platform {
            id,
            width:  100.0,
            height: 10.0,
            color: [0.5, 0.5, 0.5, 1.0],
            pos,
            rect:  [pos.x - 50.0, pos.y - 5.0, 100.0, 10.0],
        }
    }

    pub fn update(&mut self) {
        self.pos.y += 0.4;
        self.rect   = [self.pos.x - self.width/2.0, self.pos.y - self.height/2.0, self.width, self.height];
    }

    pub fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;
        let tranny = c.transform.trans(ORIGIN[0], ORIGIN[1]);
        rectangle(
            self.color,
            self.rect,
            tranny,
            gl
        );
        if DEBUG {
            line(
                RED,
                1.0,
                [self.pos.x, self.pos.y + 100.0, self.pos.x, self.pos.y - 100.0],
                tranny,
                gl
            );
//            println!("GREEN line is at {}", self.pos.x - self.width/2.0);
            line(
                GREEN,
                1.0,
                [self.pos.x - self.width/2.0, self.pos.y + 20.0, self.pos.x - self.width/2.0, self.pos.y - 20.0],
                tranny,
                gl
            );
//            println!("BLUE line is at {}", self.pos.x + self.width/2.0);
            line(
                BLUE,
                1.0,
                [self.pos.x + self.width/2.0, self.pos.y + 20.0, self.pos.x + self.width/2.0, self.pos.y - 20.0],
                tranny,
                gl
            );
        }
    }

}