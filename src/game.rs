use std::f32::consts::PI;

use glam::DVec2;
use glutin_window::GlutinWindow;
use graphics::glyph_cache;
use graphics::line::Shape;
use opengl_graphics::GlGraphics;
use opengl_graphics::GlyphCache;
use piston::Size;
use piston::Window;
use piston::{Button, ButtonArgs, ButtonState, Key, MouseButton, RenderArgs, UpdateArgs};
use piston_window::text;
use piston_window::{G2dTextureContext, Glyphs, PistonWindow};
use rusttype::point;
use rusttype::Scale;

use crate::{blade::Blade, defender::Defender, menu::Menu, BLACK, WHITE};

extern crate find_folder;
pub struct Game {
    pub(crate) gl: GlGraphics, // OpenGL drawing backend.
    pub(crate) main_menu: Menu,
    pub(crate) is_over: bool,
    pub(crate) restart: bool,
    pub(crate) defenders: Vec<Defender>,
    pub(crate) blades: Vec<Blade>, // All existing blades.
    pub(crate) cursor_pos: [f64; 2],
    pub(crate) player_x: f64,
    pub(crate) player_y: f64,
    pub(crate) size: f64,
    pub(crate) rotation: f64, // Rotation of the player.
    pub(crate) axel: f64,
    pub(crate) speed_max_mod: f64,
    pub(crate) speed_x: f64,
    pub(crate) speed_y: f64,
    pub(crate) rotation_speed: f64,
    pub(crate) m_right: bool,
    pub(crate) m_left: bool,
    pub(crate) m_up: bool,
    pub(crate) m_down: bool,
    pub(crate) is_menu_open: bool,
    pub(crate) defender_create_size: f64,
    pub(crate) window_width: f64,
    pub(crate) window_height: f64,
    pub(crate) do_exit: bool,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs, glyph_cache: &mut GlyphCache) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.size);
        let rotation = self.rotation;

        self.gl.draw(args.viewport(), |c, gl| {
            self.window_width = c.get_view_size()[0];
            self.window_height = c.get_view_size()[1];

            if !self.is_menu_open {
                // Clear the screen.
                clear(BLACK, gl);
                let transform = c
                    .transform
                    .trans(self.player_x, self.player_y)
                    .rot_rad(rotation)
                    .trans(-self.size / 2.0, -self.size / 2.0);

                for i in &self.defenders {
                    let transform = c
                        .transform
                        .trans(i.position.x, i.position.y)
                        .rot_deg(i.rotation)
                        .trans(-i.size / 2.0, -i.size / 2.0);

                    rectangle(WHITE, rectangle::square(0.0, 0.0, i.size), transform, gl);
                }

                for i in &self.blades {
                    let transform = c
                        .transform
                        .trans(i.x, i.y)
                        .rot_deg(i.rotation)
                        .trans(-i.size / 2.0, -i.size / 2.0);

                    rectangle(WHITE, rectangle::square(0.0, 0.0, i.size), transform, gl);
                }

                rectangle(WHITE, square, transform, gl);
            } else {
                let width = glyph_cache.width(56, &self.main_menu.title).unwrap();
                let width_b1 = glyph_cache.width(42, &self.main_menu.buttons[0]).unwrap();
                let width_b2 = glyph_cache.width(42, &self.main_menu.buttons[2]).unwrap();

                clear(self.main_menu.bg_fill, gl);
                let transform = c
                    .transform
                    .trans(c.get_view_size()[0] / 2.0 - width / 2.0, 150.0);

                let transform_b1 = c
                    .transform
                    .trans(c.get_view_size()[0] / 2.0 - width_b1 / 2.0, 250.0);
                let transform_b2 = c
                    .transform
                    .trans(c.get_view_size()[0] / 2.0 - width_b2 / 2.0, 330.0);

                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 56)
                    .draw(
                        &self.main_menu.title,
                        glyph_cache,
                        &DrawState::default(),
                        transform,
                        gl,
                    )
                    .unwrap();

                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 42)
                    .draw(
                        &self.main_menu.buttons[0],
                        glyph_cache,
                        &DrawState::default(),
                        transform_b1,
                        gl,
                    )
                    .unwrap();

                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 42)
                    .draw(
                        &self.main_menu.buttons[2],
                        glyph_cache,
                        &DrawState::default(),
                        transform_b2,
                        gl,
                    )
                    .unwrap();
            }
        });
    }

    pub fn controll(&mut self, args: &ButtonArgs) {
        if args.button == Button::Keyboard(Key::Q) {
            if args.state == ButtonState::Press {
                match self.is_menu_open {
                    true => self.is_menu_open = false,
                    false => {
                        self.is_menu_open = true;
                        self.speed_x = 0.0;
                        self.speed_y = 0.0;
                        self.m_down = false;
                        self.m_up = false;
                        self.m_left = false;
                        self.m_right = false;
                    }
                }
            }
        }

        if !self.is_menu_open {
            if args.button == Button::Keyboard(Key::A) {
                match args.state {
                    ButtonState::Press => self.m_left = true,
                    ButtonState::Release => self.m_left = false,
                }
            }

            if args.button == Button::Keyboard(Key::W) {
                match args.state {
                    ButtonState::Press => self.m_up = true,
                    ButtonState::Release => self.m_up = false,
                }
            }

            if args.button == Button::Keyboard(Key::S) {
                match args.state {
                    ButtonState::Press => self.m_down = true,
                    ButtonState::Release => self.m_down = false,
                }
            }

            if args.button == Button::Keyboard(Key::D) {
                match args.state {
                    ButtonState::Press => self.m_right = true,
                    ButtonState::Release => self.m_right = false,
                }
            }

            if args.button == Button::Keyboard(Key::R) && args.state == ButtonState::Release {
                self.restart = true;
            }

            if args.button == Button::Mouse(MouseButton::Left) && args.state == ButtonState::Release
            {
                //create new blade
                println!("LBM is pressed!");
                let p_pos = DVec2::new(self.player_x, self.player_y);
                let pos = DVec2::new(self.cursor_pos[0], self.cursor_pos[1]);

                let direction = pos - p_pos;
                let angle = f64::atan2(direction.y, direction.x) * 180.0 / f64::from(PI);
                self.blades.push(Blade {
                    x: self.player_x,
                    y: self.player_y,
                    size: 10.0,
                    speed: 300.0,
                    rotation: angle,
                    direction: direction.normalize(),
                });
            }

            if args.button == Button::Mouse(MouseButton::Right)
                && args.state == ButtonState::Release
            {
                println!("RBM is pressed!");
                let pos = DVec2::new(self.cursor_pos[0], self.cursor_pos[1]);

                self.defenders.push(Defender {
                    position: pos,
                    size: self.defender_create_size,
                    rotation: 0.0,
                    speed: 80.0 * 15.0 / self.defender_create_size,
                    vec_speed: DVec2::new(0.0, 0.0),
                });
            }

            if args.button == Button::Keyboard(Key::Up) && args.state == ButtonState::Release {
                self.defender_create_size += 1.0;
            }
            if args.button == Button::Keyboard(Key::Down) && args.state == ButtonState::Release {
                self.defender_create_size -= 1.0;
            }
        } else {
            if self.cursor_pos[1] > 250.0 && self.cursor_pos[1] < 300.0 {
                self.main_menu.is_resume_b_hover = true;
            } else {
                self.main_menu.is_resume_b_hover = false;
            }

            if self.cursor_pos[1] > 300.0 && self.cursor_pos[1] < 350.0 {
                self.main_menu.is_exit_b_hover = true;
            } else {
                self.main_menu.is_exit_b_hover = false;
            }

            if args.button == Button::Mouse(MouseButton::Left) && args.state == ButtonState::Release
            {
                println!("LBM is pressed!");

                if self.cursor_pos[1] > 250.0 && self.cursor_pos[1] < 300.0 {}

                if self.cursor_pos[1] > 300.0 && self.cursor_pos[1] < 350.0 {
                    self.do_exit = true;
                }
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if !self.is_menu_open {
            //let mut new_blades: Vec<Blade> = vec![];

            self.rotation += self.rotation_speed * args.dt;

            if self.restart {
                self.defenders.clear();
                self.blades.clear();

                if self.is_over {
                    self.is_over = false;
                }

                self.rotation_speed = 8.0;

                self.player_x = self.window_width / 2.0;
                self.player_y = self.window_height / 2.0;

                self.restart = false;
            }

            if !self.is_over {
                // movement after controlls detection
                if self.m_left {
                    self.speed_x -= self.axel * args.dt;
                    if self.speed_x <= -self.speed_max_mod {
                        self.speed_x = -self.speed_max_mod;
                    }
                }

                if self.m_up {
                    self.speed_y -= self.axel * args.dt;
                    if self.speed_y <= -self.speed_max_mod {
                        self.speed_y = -self.speed_max_mod;
                    }
                }

                if self.m_down {
                    self.speed_y += self.axel * args.dt;
                    if self.speed_y >= self.speed_max_mod {
                        self.speed_y = self.speed_max_mod;
                    }
                }

                if self.m_right {
                    self.speed_x += self.axel * args.dt;
                    if self.speed_x >= self.speed_max_mod {
                        self.speed_x = self.speed_max_mod;
                    }
                }

                if !self.m_right && !self.m_left {
                    if self.speed_x > 0.0 {
                        self.speed_x -= self.axel * args.dt;
                    }
                    if self.speed_x < 0.0 {
                        self.speed_x += self.axel * args.dt;
                    }
                }

                if !self.m_up && !self.m_down {
                    if self.speed_y > 0.0 {
                        self.speed_y -= self.axel * args.dt;
                    }
                    if self.speed_y < 0.0 {
                        self.speed_y += self.axel * args.dt;
                    }
                }

                self.player_x += self.speed_x * args.dt;
                self.player_y += self.speed_y * args.dt;
            } else {
                // die motion
                self.rotation_speed -= self.rotation_speed / 1.1 * args.dt;
                if self.rotation_speed <= 0.0 {
                    self.rotation_speed = 0.0;
                }
            }

            for i in &mut self.defenders {
                let pdistance = i.position - DVec2::new(self.player_x, self.player_y);
                if pdistance.length() < (self.size / 2.0 + i.size / 2.0) {
                    self.is_over = true;
                }

                i.move_to_player(self.player_x, self.player_y, &args);
            }

            let mut extra_blades_index: Vec<i32> = vec![]; // All extra blades` indexes

            let mut p = -1; // This is an index of an extra blade inside an array. -1 bc arrays starts from 0

            for i in &mut self.blades {
                i.move_forward(&args);
                p += 1;

                let pdistance = DVec2::new(i.x, i.y) - DVec2::new(self.player_x, self.player_y);
                if pdistance.length() > 5000.0 {
                    extra_blades_index.push(p);
                }
            }

            for i in extra_blades_index {
                self.blades.remove(i.try_into().unwrap());
            }
        }
    }
}
