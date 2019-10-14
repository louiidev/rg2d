use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, WindowCanvas};
use std::path::Path;
use std::time::Duration;


pub struct Rg2d {}

impl Rg2d {

  pub fn new() -> (WindowCanvas, EventPump) {
      let sdl_context = sdl2::init().unwrap();
      let video_subsystem = sdl_context.video().unwrap();
      let even_pump = sdl_context.event_pump().unwrap();
      let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
      let window = video_subsystem
      .window("rg2d", 800, 600)
      .build()
      .expect("could not initialize video subsystem");
      let canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");
      (canvas, even_pump)
  }

}