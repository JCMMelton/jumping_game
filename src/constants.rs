
extern crate nalgebra;

use self::nalgebra::core::{ Vector2 };
use std::f64;
use util::*;

pub const DEBUG: bool = false;

pub const WIDTH:  f64 = 800.0;
pub const HEIGHT: f64 = 600.0;

pub const ORIGIN: [f64;2] = [WIDTH/2.0, HEIGHT/2.0];

pub const PI: f64 = f64::consts::PI;

pub const CORRECTION: f64 = PI*0.5;

pub const HPI: f64 = PI*0.5;

pub const DOWN:  f64 = CORRECTION;
pub const UP:    f64 = -DOWN;
pub const LEFT:  f64 = PI;
pub const RIGHT: f64 = 0.0;

pub const GRAVITY_PULL:   f64 = 0.00981;
pub const PLATFORM_SPEED: f64 = 0.4;

pub const RED:    [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE:   [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const GREEN:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub const YELLOW: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
pub const TEAL:   [f32; 4] = [1.0, 1.0, 0.0, 1.0];
