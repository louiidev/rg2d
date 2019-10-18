use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;


use std::env;
use std::borrow::Borrow;


pub struct Context {
    pub canvas: Canvas<sdl2::video::Window>,
    pub texture_creator: TextureCreator<sdl2::video::WindowContext>,
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
