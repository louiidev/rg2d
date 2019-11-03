use sdl2::image::{self, InitFlag};
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::rect::Point;
use sdl2::render::{TextureCreator};
use std::collections::HashSet;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::mouse::MouseState;


pub struct Camera {
    pub position: Point
}

impl Camera {
    fn default() -> Self {
        Camera {
            position: Point::new(0, 0)
        }
    }
}

pub struct Input {
    pub keys_current: HashSet<Keycode>,
    keys_down: HashSet<Keycode>,
    keys_up: HashSet<Keycode>,
    mouse_state: MouseState,
}

impl Input {
    fn new() -> Self {
        Input {
            keys_current: HashSet::new(),
            keys_down: HashSet::new(),
            keys_up: HashSet::new(),
            mouse_state: MouseState::from_sdl_state(0)
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

    pub fn set_mouse_state(&mut self, events: &EventPump) {
        let state = events.mouse_state();
        self.mouse_state = state;
    }

    pub fn set_keys(&mut self, events: &EventPump) {
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        let new_keys = &keys - &self.keys_current;
        let old_keys = &self.keys_current - &keys;
        self.keys_down = new_keys;
        self.keys_up = old_keys;
        self.keys_current = keys;
    }

    pub fn get_mouse_pos(&mut self) -> Point {
        Point::new(self.mouse_state.x(), self.mouse_state.y())
    }
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub canvasScale: f32,
    pub title: String
}


fn parse_config(config: Option<Config>) -> Config {
    match config {
        Some(res) => res,
        None => Config {
            width: 1200,
            height: 800,
            canvasScale: 0.5,
            title: "rg2d".to_string()
        }
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
    pub fn new(config: Option<Config>) -> (Context, EventPump) {
        let _config = parse_config(config);
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
        let window = video_subsystem
            .window(&_config.title, _config.width, _config.height)
            .opengl()
            .resizable()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .expect("could not make a canvas");
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        canvas.window().gl_set_context_to_current();
        let texture_creator = canvas.texture_creator();
        let input = Input::new();
        let camera = Camera::default();
        canvas.set_scale(_config.canvasScale, _config.canvasScale);
        (Context { canvas, texture_creator, input, tff: ttf_context, camera }, sdl_context.event_pump().unwrap())
    }}
