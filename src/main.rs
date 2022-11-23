mod blade;
mod defender;
mod game;
mod menu;

use crate::blade::Blade;
use crate::defender::Defender;
use crate::game::Game;
use crate::menu::Menu;

use std::f32::consts::PI;

use graphics::CharacterCache;
use piston_window::{Glyphs, PistonWindow, Transformed};
use rand::Rng;

use glam::DVec2;
use glutin_window::GlutinWindow;
use opengl_graphics::GlyphCache;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{
    ButtonArgs, ButtonEvent, ButtonState, EventLoop, Key, MouseButton, MouseCursorEvent, Window,
    WindowSettings,
};

use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

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

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("BALLER", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .build()
        .unwrap();

    let main_menu: Menu = Menu {
        bg_fill: [0.0, 0.0, 0.0, 1.0],
        title: "Monochrom".to_string(),
        buttons: [
            "Resume".to_string(),
            "Options".to_string(),
            "Exit".to_string(),
        ],
        is_resume_b_hover: false,
        is_options_b_hover: false,
        is_exit_b_hover: false,
    };

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("Hackbot.otf");
    let mut glyph_cache = GlyphCache::new(font, (), TextureSettings::new()).unwrap();

    //let mut glyphs = window.load_font(font).unwrap();

    let defenders = create_defenders(10);

    let blades: Vec<Blade> = vec![];

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        main_menu: main_menu,
        window_width: window.size().width,
        window_height: window.size().height,
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
        is_menu_open: false,
        defender_create_size: 15.0,
        do_exit: false,
    };

    let mut events = Events::new(EventSettings::new().lazy(false));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.mouse_cursor_args() {
            game.cursor_pos = args;
        }

        if let Some(args) = e.render_args() {
            game.render(&args, &mut glyph_cache);
        }

        if let Some(args) = e.button_args() {
            game.controll(&args); // player controls
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if game.do_exit {
            return;
        }
    }
}
