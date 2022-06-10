extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::f32::consts::PI;

use std::{
    cell::{Ref, RefCell},
    rc::Rc
};

use rand::Rng;

use glam::{DVec2, Vec2};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Window, Event, ButtonEvent, ButtonArgs, Key, ButtonState};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Button};
use piston::window::WindowSettings;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct Defender {
    id: u32,
    position: DVec2,
    size: f64,
    rotation: f64,
    speed: f64,
    vec_speed: DVec2
}

impl Defender {
    pub fn move_to_player(&mut self, player_x: f64, player_y: f64, args: &UpdateArgs){

        let p_pos = DVec2::new(player_x, player_y);
        let mut pos = DVec2::new(self.position.x, self.position.y);
        
        let direction = p_pos - pos;
        let angle = f64::atan2(direction.y, direction.x) * 180.0/f64::from(PI);
        self.rotation = angle;
        self.vec_speed = direction.normalize() * self.speed * args.dt;
        let res_pos = pos + self.vec_speed;
        self.position = res_pos;
    }
}

fn create_defenders(count: u32) -> Vec<Defender> {

    let mut rng = rand::thread_rng();

    (0..count).map( |x| Defender {
        id: x,
        position: DVec2::new(rng.gen_range(0..800) as f64, rng.gen_range(0..600) as f64),
        size: 15.0,
        rotation: 0.0,
        speed: 80.0,
        vec_speed: DVec2::new(0.0, 0.0)
    }).collect()
}

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    is_over: bool,
    defenders: Vec<Defender>,
    player_x: f64, player_y: f64,
    size: f64,
    rotation: f64,  // Rotation for the player.
    speed: f64,
    rotation_speed: f64,
    m_right: bool,
    m_left: bool,
    m_up: bool,
    m_down: bool
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
                .trans(-self.size/2.0, -self.size/2.0);

            for i in &self.defenders {
                let transform = c
                .transform
                .trans(i.position.x, i.position.y)
                .rot_deg(i.rotation)
                .trans(-i.size/2.0, -i.size/2.0);

            rectangle(RED, rectangle::square(0.0, 0.0, i.size), transform, gl);
            }

            rectangle(RED, square, transform, gl);
        });
    }

    fn controll(&mut self, args: &ButtonArgs){
        if args.button == Button::Keyboard(Key::A) 
        {
            match args.state {
                ButtonState::Press => self.m_left = true,
                ButtonState::Release => self.m_left = false
            }
        }

        if args.button == Button::Keyboard(Key::W) 
        {
            match args.state {
                ButtonState::Press => self.m_up = true,
                ButtonState::Release => self.m_up = false
            }
        }

        if args.button == Button::Keyboard(Key::S) 
        {
            match args.state {
                ButtonState::Press => self.m_down = true,
                ButtonState::Release => self.m_down = false
            }
        }

        if args.button == Button::Keyboard(Key::D) 
        {
            match args.state {
                ButtonState::Press => self.m_right = true,
                ButtonState::Release => self.m_right = false
            }
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 5 radians per second.

        self.rotation += self.rotation_speed * args.dt;
        if !self.is_over{
            if self.m_left == true {
                self.player_x -= self.speed * args.dt;
            }

            if self.m_up == true {
               self.player_y -= self.speed * args.dt;
            }

            if self.m_down == true {
                self.player_y += self.speed * args.dt;
            }

            if self.m_right == true {
                self.player_x += self.speed * args.dt;
            }
        }else{
            self.rotation_speed -= self.rotation_speed/1.1 * args.dt;
            if self.rotation_speed <= 0.0 { self.rotation_speed = 0.0; }
        }

        for i in &mut self.defenders {

            let pdistance = i.position - DVec2::new(self.player_x, self.player_y);
            if pdistance.length() < (self.size/2.0 + i.size/2.0) {
                self.is_over = true;
            }

            i.move_to_player(self.player_x, self.player_y, &args);
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
    let defenders:Vec<Defender> = create_defenders(10); // list of all creatures

    //defenders.into_iter().map(|x|, if);

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        is_over: false,
        defenders: defenders,
        player_x:window.size().width/2.0,// player`s start position
        player_y:window.size().height/2.0,//
        size:40.0,
        rotation: 0.0,
        speed: 300.0,
        rotation_speed: 8.0,
        m_right: false,
        m_left: false,
        m_up: false,
        m_down: false
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.button_args() {
            game.controll(&args);// player controls
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}