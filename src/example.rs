extern crate gl;
extern crate sdl2;

mod lib;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, WindowCanvas};
use std::path::Path;
use std::time::Duration;


enum Direction {
  Up,
  Right,
  Down,
  Left,
}

struct Player {
  position: Point,
  sprite: Rect,
  speed: i32,
  direction: Direction,
}

fn render_sprite(
  canvas: &mut WindowCanvas,
  texture: &Texture,
  player: &Player,
) -> Result<(), String> {
  let (width, height) = canvas.output_size()?;
  let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
  let screen_rect = Rect::from_center(
    screen_position,
    player.sprite.width(),
    player.sprite.height(),
  );

  canvas.copy(texture, player.sprite, screen_rect)?;
  Ok(())
}

fn main() -> Result<(), String> {
  let (mut canvas, mut event_pump) = main::Rg2d::new();

  let texture_creator = canvas.texture_creator();
  let path = Path::new("assets/bardo.png");
  let texture = texture_creator.load_texture(path).unwrap();

  let mut player_data = Player {
    position: Point::new(0, 0),
    sprite: Rect::new(0, 0, 26, 36),
    speed: 5,
    direction: Direction::Right,
  };

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
          player_data.position = player_data.position.offset(-player_data.speed, 0);
        }
        Event::KeyDown {
          keycode: Some(Keycode::D),
          ..
        } => {
          player_data.position = player_data.position.offset(player_data.speed, 0);
        }
        Event::KeyDown {
          keycode: Some(Keycode::W),
          ..
        } => {
          player_data.position = player_data.position.offset(0, -player_data.speed);
        }
        Event::KeyDown {
          keycode: Some(Keycode::S),
          ..
        } => {
          player_data.position = player_data.position.offset(0, player_data.speed);
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