use sdl2::image::{self, InitFlag};
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::render::{TextureCreator};
use std::collections::HashSet;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Sdl2TtfContext;
use crate::physics::Vector2;


pub struct Camera {
    pub position: Vector2
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector2::default()
        }
    }
}


pub struct Input {
    pub keys_current: HashSet<Keycode>,
    keys_down: HashSet<Keycode>,
    keys_up: HashSet<Keycode>
}

impl Input {
    fn new() -> Self {
        Input {
            keys_current: HashSet::new(),
            keys_down: HashSet::new(),
            keys_up: HashSet::new()
        }
    }
}

impl Input {
    pub fn get_key_down(&self, key: Keycode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn get_key(&self, key: Keycode) -> bool {
        self.keys_current.contains(&key)
    }

    pub fn get_key_up(&self, key: Keycode) -> bool {
        self.keys_up.contains(&key)
    }

    pub fn set_keys(&mut self, events: &EventPump) {
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        let new_keys = &keys - &self.keys_current;
        let old_keys = &self.keys_current - &keys;
        self.keys_down = new_keys;
        self.keys_up = old_keys;
        self.keys_current = keys;
    }
}

pub struct Context {
    pub canvas: Canvas<sdl2::video::Window>,
    pub texture_creator: TextureCreator<sdl2::video::WindowContext>,
    pub input: Input,
    pub tff: Sdl2TtfContext,
    pub camera: Camera
}

impl Context {
    pub fn new() -> (Context, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
        let window = video_subsystem
            .window("rg2d", 800, 600)
            .build()
            .expect("could not initialize video subsystem");
        let canvas = window
            .into_canvas()
            .software()
            .build()
            .expect("could not make a canvas");
        let texture_creator = canvas.texture_creator();
        let input = Input::new();
        let camera = Camera::new();
        (Context { canvas, texture_creator, input, tff: ttf_context, camera }, sdl_context.event_pump().unwrap())
    }}
