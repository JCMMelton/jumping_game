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

struct Block {
    color: [f32;4],
    rect: [f64; 4],
    pos: Vector2<f64>,
    speed: f64,
}

pub struct Background {
    blocks: Vec<Vec<Block>>
}

impl Background {

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut blocks = Vec::new();
        let l:  i32 = 5;
        let l64: f64 = (l) as f64;
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut w = 40.0  + 50.0*l64;
        let mut h = 40.0  + 75.0*l64;
        let mut s: f64 = 0.05;
        let mut c: f32 = 0.0;
        let mut i: i32 = 20;
        let p_w: f64 = 100.0;
        let p_h: f64 = 1000.0;
        let p_width  = WIDTH  + p_w;
        let p_height = HEIGHT + p_h;
        let p_ow = ORIGIN[0] + (p_w/2.0);
        let p_oh = ORIGIN[1] + (p_h/2.0);
        for zz in 0..l {
            let mut layer = Vec::new();
            c += 0.01;
            w -= 50.0;
            h -= 55.0;
            s  = 0.1/((l - zz) as f64);
            i += 10;
            for z in 0..i {
                x = (rng.gen::<f64>() * p_width)  - p_ow;
                y = (rng.gen::<f64>() * p_height) - p_oh;
//                let rw = (rng.gen::<f64>()+0.1) * 1.4;
//                let rh = (rng.gen::<f64>()+0.1) * 1.3;
                let rd: f64 = rng.gen::<f64>();
                let rw: f64 = rd + 0.5;
                let rh: f64 = 1.5 - rd;

                layer.push(Block{
                    color: [c, c, c, 1.0],
                    rect: [-(w/2.0), -(h/2.0), w*rw, h*rh],
                    pos: Vector2::new(x, y),
                    speed: s
                });
            }
            blocks.push(layer);
        }
        Background {
            blocks
        }
    }

    pub fn update(&mut self) {
        for layer in self.blocks.iter_mut() {
            for block in layer.iter_mut() {
                block.pos.y += block.speed;
                if block.pos.y  >   ORIGIN[1] + 500.0 {
                    block.pos.y = -(ORIGIN[1] + 500.0);
                }
            }
        }
    }

    pub fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;
        let tranny = c.transform.trans(ORIGIN[0], ORIGIN[1]);
        for layer in self.blocks.iter() {
            for block in layer.iter() {
                rectangle(
                    block.color,
                    block.rect,
                    tranny.trans(block.pos.x, block.pos.y),
                    gl
                );
            }
        }
    }

}