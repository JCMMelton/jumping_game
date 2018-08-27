
extern crate nalgebra;

use self::nalgebra::core::{ Vector2 };

use constants::*;

pub fn to_vector(magnitude: f64, angle: f64) -> Vector2<f64> {
    let x: f64 = f64::cos(angle) * magnitude;
    let y: f64 = f64::sin(angle) * magnitude;
    Vector2::new(x, y)
}
pub fn gravity() -> Vector2<f64> {
    to_vector(GRAVITY_PULL, DOWN)
}