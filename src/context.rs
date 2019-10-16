use sdl2::image::{self, InitFlag};
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct Context {
    pub canvas: Canvas<sdl2::video::Window>,
    pub texture_creator: TextureCreator<WindowContext>
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
            .software()
            .build()
            .expect("could not make a canvas");
        let texture_creator = canvas.texture_creator();
        (Context { canvas, texture_creator }, sdl_context.event_pump().unwrap())
    }}
