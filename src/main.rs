extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f32::consts::PI;

/*use std::{
    cell::{Ref, RefCell},
    rc::Rc
};*/

use rand::Rng;

use glam::DVec2;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{
    ButtonArgs, /*Event,*/ ButtonEvent, ButtonState, EventLoop, Key, MouseButton,
    MouseCursorEvent, Window,
};

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

#[derive(Clone, Debug)]
pub struct Defender {
    position: DVec2,
    size: f64,
    rotation: f64,
    speed: f64,
    vec_speed: DVec2,
}

impl Defender {
    pub fn collide(&self, _colliding_blade: &Blade) {}

    pub fn evolve() {}

    pub fn move_to_player(&mut self, player_x: f64, player_y: f64, args: &UpdateArgs) {
        let p_pos = DVec2::new(player_x, player_y);
        let pos = DVec2::new(self.position.x, self.position.y);

        let direction = p_pos - pos;
        let angle = f64::atan2(direction.y, direction.x) * 180.0 / f64::from(PI);
        self.rotation = angle;
        self.vec_speed = direction.normalize() * self.speed * args.dt;
        let res_pos = pos + self.vec_speed;
        self.position = res_pos;
    }
}

impl Drop for Defender {
    fn drop(&mut self) {
        println!("Defender is dropped!");
    }
}

fn create_defenders(count: u32) -> Vec<Defender> {
    let mut rng = rand::thread_rng();

    (0..count)
        .map(|_| Defender {
            position: DVec2::new(rng.gen_range(0..800) as f64, rng.gen_range(0..600) as f64),
            size: 15.0,
            rotation: 0.0,
            speed: 80.0,
            vec_speed: DVec2::new(0.0, 0.0),
        })
        .collect()
}

pub struct Blade {
    x: f64,
    y: f64,
    size: f64,
    speed: f64,
    rotation: f64,
    direction: DVec2,
}

impl Blade {
    fn move_forward(&mut self, args: &UpdateArgs) {
        let pos = DVec2::new(self.x, self.y);
        let vec_speed = self.direction * self.speed * args.dt;
        let res_pos = pos + vec_speed;
        self.x = res_pos[0];
        self.y = res_pos[1];
        self.speed += 3.0 * self.speed * args.dt;
    }
}

impl Drop for Blade {
    fn drop(&mut self) {
        println!("Blade is dropped!");
    }
}

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    is_over: bool,
    restart: bool,
    defenders: Vec<Defender>,
    blades: Vec<Blade>, // All blades in the game
    cursor_pos: [f64; 2],
    player_x: f64,
    player_y: f64,
    size: f64,
    rotation: f64, // Rotation for the player.
    axel: f64,
    speed_max_mod: f64,
    speed_x: f64,
    speed_y: f64,
    rotation_speed: f64,
    m_right: bool,
    m_left: bool,
    m_up: bool,
    m_down: bool,
    defender_create_size: f64,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.size);
        let rotation = self.rotation;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

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

                rectangle(RED, rectangle::square(0.0, 0.0, i.size), transform, gl);
            }

            for i in &self.blades {
                let transform = c
                    .transform
                    .trans(i.x, i.y)
                    .rot_deg(i.rotation)
                    .trans(-i.size / 2.0, -i.size / 2.0);

                rectangle(RED, rectangle::square(0.0, 0.0, i.size), transform, gl);
            }

            rectangle(RED, square, transform, gl);
        });
    }

    fn controll(&mut self, args: &ButtonArgs) {
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

        if args.button == Button::Mouse(MouseButton::Left) && args.state == ButtonState::Release {
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

        if args.button == Button::Mouse(MouseButton::Right) && args.state == ButtonState::Release {
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
    }

    fn update(&mut self, args: &UpdateArgs) {
        //let mut new_blades: Vec<Blade> = vec![];

        self.rotation += self.rotation_speed * args.dt;

        if self.restart {
            self.defenders.clear();
            self.blades.clear();

            if self.is_over {
                self.is_over = false;
            }

            self.rotation_speed = 8.0;

            self.player_x = 800.0 / 2.0;
            self.player_y = 600.0 / 2.0;

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

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("spinning-square", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .build()
        .unwrap();

    // Fill in defenders list
    let defenders: Vec<Defender> = create_defenders(10); // list of all creatures

    //defenders.into_iter().map(|x|, if);

    let blades: Vec<Blade> = vec![];

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        is_over: false,
        restart: false,
        defenders: defenders,
        blades: blades,
        cursor_pos: [0.0, 0.0],
        player_x: window.size().width / 2.0, // player`s start position
        player_y: window.size().height / 2.0, //
        size: 40.0,
        rotation: 0.0,
        axel: 2300.0,
        speed_max_mod: 300.0,
        speed_x: 0.0,
        speed_y: 0.0,
        rotation_speed: 8.0,
        m_right: false,
        m_left: false,
        m_up: false,
        m_down: false,
        defender_create_size: 15.0,
    };

    let mut events = Events::new(EventSettings::new().lazy(false));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            game.cursor_pos = args;
        }

        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.button_args() {
            game.controll(&args); // player controls
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
