extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate nalgebra;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use self::nalgebra::core::{ Vector2 };

use constants::*;
use player::*;
use platform::*;
use background::*;

pub struct Game {
    pub gl:         GlGraphics,
    pub player:     Player,
    pub alive:      bool,
    pub game_time:  f64,
    pub plat_time:  f64,
    pub platforms: Vec<Platform>,
    pub platform_id: u32,
    pub background: Background,
}

impl Game {

    pub fn new(opengl: OpenGL) -> Self {
        Game {
            gl: GlGraphics::new(opengl),
            player: Player::new(),
            alive: true,
            game_time: 0.0,
            plat_time: 0.0,
            platforms: Vec::new(),
            platform_id: 0,
            background: Background::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let background: &Background = &self.background;
        let player: &Player = &self.player;
        let platforms: &Vec<Platform> = &self.platforms;
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);
            let tranny = c.transform.trans(ORIGIN[0], ORIGIN[1]);
            background.render(c, gl);
            player.render(c, gl);
            for platform in platforms.iter() {
                platform.render(c, gl);
            }
            if DEBUG {
                line( // DOWN
                      [1.0, 0.0, 0.0, 1.0],
                      1.0,
                      [0.0, 0.0, 100.0, 100.0],
                      tranny,
                      gl
                );
                line( // LEFT
                      [1.0, 1.0, 0.0, 1.0],
                      1.0,
                      [0.0, 0.0, 100.0, 100.0],
                      tranny.rot_rad(PI*0.5),
                      gl
                );
                line( // UP
                      [0.0, 1.0, 1.0, 1.0],
                      1.0,
                      [0.0, 0.0, 100.0, 100.0],
                      tranny.rot_rad(PI),
                      gl
                );
                line( // RIGHT
                      [0.0, 0.0, 1.0, 1.0],
                      1.0,
                      [0.0, 0.0, 100.0, 100.0],
                      tranny.rot_rad(PI*1.5),
                      gl
                );
                let o = Vector2::new(0.0, 0.0);
                let t = Vector2::new(100.0, 100.0);
                let v = o + t;
                line( // TEST
                      [0.5, 0.0, 0.5, 1.0],
                      1.0,
                      [0.0, 0.0, v.x, v.y],
                      tranny,
                      gl
                );
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.alive = self.player.alive;
        if self.alive {
            self.player.update(args.dt, &self.platforms);
            self.game_time += args.dt;
            self.plat_time += args.dt;
            if self.plat_time > 1.0 {
                self.plat_time    = 0.0;
                self.platform_id += 1;
                self.platforms.push(Platform::new(self.platform_id));
            }
            for platform in self.platforms.iter_mut() {
                platform.update();
            }
            self.platforms.retain(|p| p.pos.y < (HEIGHT/2.0)+20.0);
            self.background.update();
        }
    }

    pub fn on_resize(&mut self, new_dimensions: &[u32; 2]) {}

    pub fn reset(&mut self) {
        self.alive        = true;
        self.platform_id += 1;
        self.platforms    = vec![Platform::new_at_location(0.0, -170.0, self.platform_id)];
        self.player.reset();
    }

    pub fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Return => {
                if !self.alive {
                    self.reset();
                }
            }
            Key::Space => {
                self.player.jump();
            }
            Key::A => {
                self.player.move_left(true);
            }
            Key::D => {
                self.player.move_right(true);
            }
            _ => {}
        }
    }

    pub fn on_key_release(&mut self, key: Key) {
        match key {
            Key::A => {
                self.player.move_left(false);
            }
            Key::D => {
                self.player.move_right(false);
            }
            _ => {}
        }
    }

    pub fn on_mouse_mov(&mut self, motion: &[f64; 2]) {}

    pub fn on_mouse_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
            }
            _ => {}
        }
    }

    pub fn on_mouse_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
            }
            _ => {}
        }
    }

}