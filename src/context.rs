use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::rect::Point;
use sdl2::video::Window;
use sdl2::EventPump;
use std::collections::HashSet;


pub struct Input {
    pub keys_current: HashSet<Keycode>,
    pub keys_down: HashSet<Keycode>,
    keys_up: HashSet<Keycode>,
    mouse_state: MouseState,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys_current: HashSet::new(),
            keys_down: HashSet::new(),
            keys_up: HashSet::new(),
            mouse_state: MouseState::from_sdl_state(0),
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
        let keys = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
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

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub canvas_scale: f32,
    pub title: String,
}

fn parse_config(config: Option<Config>) -> Config {
    match config {
        Some(res) => res,
        None => Config {
            width: 1200,
            height: 800,
            canvas_scale: 0.5,
            title: "rg2d".to_string(),
        },
    }
}

pub struct Context {
    pub window: Window,
    pub input: Input,
    pub gl_context: sdl2::video::GLContext,
}

impl Context {
    pub fn new(config: Option<Config>) -> (Context, EventPump) {
        let _config = parse_config(config);
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 1);

        let window = video_subsystem
            .window("Game", _config.width, _config.height)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let input = Input::new();
        (
            Context {
                window,
                input,
                gl_context,
            },
            sdl.event_pump().unwrap(),
        )
    }
}
