use sdl2::image::{self, InitFlag};
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Context {
    pub canvas: WindowCanvas
}

impl Context {
    pub fn new() -> (Context, EventPump) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
        let window = video_subsystem
            .window("rg2d", 800, 600)
            .build()
            .expect("could not initialize video subsystem");
        let canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");
        (Context { canvas }, sdl_context.event_pump().unwrap())
    }}
