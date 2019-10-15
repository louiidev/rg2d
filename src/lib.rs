pub mod context;
pub mod rb2d;

use sdl2::EventPump;
use sdl2::image::{self, InitFlag};
use sdl2::render::{WindowCanvas};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::path::Path;

use crate::context::Context;

fn main() -> Result<(), String> {
  let (mut canvas, mut event_pump) = rb2d::Rg2d::new();

  let texture_creator = canvas.texture_creator();
  let path = Path::new("assets/bardo.png");
  // let texture = texture_creator.load_texture(path).unwrap();


  'main: loop {
    for event in event_pump.poll_iter() {
      match event {
        sdl2::event::Event::Quit { .. } => break 'main,
        Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'main,
        Event::KeyDown {
          keycode: Some(Keycode::A),
          ..
        } => {
         // player_data.position = player_data.position.offset(-player_data.speed, 0);
        }
        Event::KeyDown {
          keycode: Some(Keycode::D),
          ..
        } => {
          //player_data.position = player_data.position.offset(player_data.speed, 0);
        }
        Event::KeyDown {
          keycode: Some(Keycode::W),
          ..
        } => {
          //player_data.position = player_data.position.offset(0, -player_data.speed);
        }
        Event::KeyDown {
          keycode: Some(Keycode::S),
          ..
        } => {
         // player_data.position = player_data.position.offset(0, player_data.speed);
        }
        _ => {}
      }
    }

    canvas.set_draw_color(Color::RGB(130, 130, 255));
    canvas.clear();
    canvas.present();

    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }

  Ok(())
}
