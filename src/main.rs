use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::path::Path;

use rg2d::components::{ Entity, Transform };
use rg2d::physics::Vector2;
use rg2d::graphics::{ Render, Sprite };
use rg2d::context::Context;
use rg2d::game_loop::{GameLoop, EventHandler};

mod graphics;
mod physics;
mod components;
mod context;
mod game_loop;

fn main() -> Result<(), String> {
  let (mut ctx, mut event_pump) = Context::new();
  ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
  ctx.canvas.clear();
  ctx.canvas.present();


  struct MyGame {
      player: Entity,
      movement_speed: i32,
  }

  impl MyGame {
    pub fn new(mut _ctx: &mut Context) -> MyGame {
    let sprite_path = Path::new("assets/bardo.png");
    let font_path = Path::new("fonts/small_pixel.ttf");
    let texture = _ctx.texture_creator.load_texture(sprite_path).unwrap();
      MyGame {
          player: Entity::new(Sprite::new(texture, Rect::new(0, 0, 26, 36)), Transform::default()),
          movement_speed: 2,
      }
    }
  }
  impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), String> {
      let mut x = 0;
      let mut y = 0;
      for key in _ctx.input.keys_current.iter() {
        match key {
          Keycode::A | Keycode::Left => x = -1,
          Keycode::D | Keycode::Right => x = 1,
          Keycode::W | Keycode::Up => y = -1,
          Keycode::S | Keycode::Down => y = 1,
          _ => {}
        }
      }
      self.player.transform.position = self.player.transform.position + Vector2::new(x * self.movement_speed, y * self.movement_speed);
      _ctx.camera.position+= Vector2::right();
      Ok(())
    }

    fn render(&mut self, mut _ctx: &mut Context) -> Result<(), String> {
        Render::clear(_ctx, Color::RGB(130, 130, 255));
        Render::sprite(_ctx, &self.player.sprite, self.player.transform.position);
        Render::rect(_ctx, Rect::new(0, 0, 50, 50), Color::RGB(0, 0, 255));
        Ok(())
    }
  }

  let mut my_game = MyGame::new(&mut ctx);

  match GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
    Ok(()) => println!("Game exited"),
    Err(e) => println!("Error occured: {}", e),
  }
  Ok(())
}
