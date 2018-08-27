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
use platform::*;

pub struct Player {
    pub alive: bool,
    pub falling: bool,
    pub jumping: bool,
    pub jumping_timer: f64,
    pub jumping_cooldown: f64,
    pub pos: Vector2<f64>,
    pub vel: Vector2<f64>,
    pub width: f64,
    pub height: f64,
    pub on_platform: bool,
    pub movement_up:    bool,
    pub movement_down:  bool,
    pub movement_left:  bool,
    pub movement_right: bool,
}

impl Player{

    pub fn new() -> Self {
        Player{
            alive:   true,
            falling: false,
            jumping: false,
            jumping_timer: 0.0,
            jumping_cooldown: 2.0,
            pos: Vector2::new(0.0, -200.0),
            vel: Vector2::new(0.0, 0.0),
            width:  8.0,
            height: 12.0,
            on_platform: false,
            movement_up:    false,
            movement_down:  false,
            movement_left:  false,
            movement_right: false,
        }
    }

    pub fn render(&self, c: graphics::Context, gl: &mut GlGraphics) {
        use self::graphics::*;
        let tranny = c.transform.trans(ORIGIN[0], ORIGIN[1]);//.rot_rad(CORRECTION);
        rectangle(
            [0.0,  ((self.jumping_timer * 0.4) + 0.2) as f32, 0.5, 1.0],
            [-(self.width/2.0), -(self.height/2.0), self.width, self.height],
            tranny.trans(self.pos.x, self.pos.y),
            gl
        );
        if DEBUG {
//            println!("{:?}", self.pos);
//            println!("player BLUE line is at {}", self.pos.x - self.width/2.0);
            line(
                BLUE,
                1.0,
                [self.pos.x - self.width/2.0, self.pos.y + 20.0, self.pos.x - self.width/2.0, self.pos.y - 20.0],
                tranny,
                gl
            );
//            println!("player GREEN line is at {}", self.pos.x + self.width/2.0);
            line(
                GREEN,
                1.0,
                [self.pos.x + self.width/2.0, self.pos.y + 20.0, self.pos.x + self.width/2.0, self.pos.y - 20.0],
                tranny,
                gl
            );
        }
    }

    pub fn update(&mut self, dt: f64, platforms: &Vec<Platform>) {
        if self.jumping_timer < self.jumping_cooldown {
            self.jumping_timer += dt*0.25;
        }
        else {
            self.jumping_timer = self.jumping_cooldown;
        }
        self.on_platform = false;
        for platform in platforms.iter() {
            let foot_l = self.pos.x - self.width/2.0;
            let foot_r = self.pos.x + self.width/2.0;
            let foot_y  = self.pos.y + self.height/2.0;

            let px_l = platform.pos.x - platform.rect[2]/2.0;
            let px_r = platform.pos.x + platform.rect[2]/2.0;
            let py_t  = platform.rect[1]-1.0;// - platform.rect[3];
            let py_b  = platform.rect[1]+2.0;// - platform.rect[3]+1.0;

            if !(foot_r < px_l || foot_l > px_r) && (foot_y > py_t && foot_y < py_b) {
                self.on_platform = true;
//                println!("ON PLATFORM {} foot_r {} < px_l {} || foot_l {} > px_r {}", platform.id, foot_r, px_l, foot_l, px_r);
            }

        }
        if self.vel.y > 0.0 {
            self.jumping = false;
            self.falling = true;
        }
        if self.on_platform && !self.jumping {
            self.jumping_timer += dt*0.95;
            if self.vel.y > 0.0 {
                self.vel.y = PLATFORM_SPEED;
            }
            if self.vel.x > 0.01 || self.vel.x < 0.01 {
                self.vel.x *= 0.99;
            }
            self.jumping = false;
            self.falling = false;
        }
        else {
            self.vel += gravity();
        }
        if self.movement_left && self.vel.x > -1.0 {
            self.vel += to_vector(0.01, LEFT);
        }
        if self.movement_right && self.vel.x < 1.0 {
            self.vel += to_vector(0.01, RIGHT);
        }

        if (self.pos.x < ((-WIDTH/2.0)+12.0) && self.vel.x < 0.0)
        || (self.pos.x > (( WIDTH/2.0)-12.0) && self.vel.x > 0.0) {
            self.vel.x *= -0.4;
        }

        self.pos += self.vel;
        if self.pos.y > HEIGHT / 2.0 {
            self.alive = false;
        }
    }

    pub fn jump(&mut self) {
//        println!("{}  jumping {}    falling {}", self.jumping_timer, self.jumping, self.falling);
        if !self.jumping && !self.falling {
            self.vel += to_vector((self.jumping_timer)+(self.vel.x*0.2), UP);
            self.jumping = true;
            self.jumping_timer = 0.0;
        }
    }

    pub fn move_left(&mut self, toggle: bool) {
        if !self.movement_right {
            self.movement_left = toggle;
        }
    }
    pub fn move_right(&mut self, toggle: bool) {
        if !self.movement_left {
            self.movement_right = toggle;
        }
    }

    pub fn reset(&mut self) {
        self.alive = true;
        self.falling = false;
        self.jumping = false;
        self.movement_left =  false;
        self.movement_right = false;
        self.pos = Vector2::new(0.0, -200.0);
        self.vel = Vector2::new(0.0, 0.0);
    }
}