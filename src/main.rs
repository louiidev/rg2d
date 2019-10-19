use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use sdl2::rect::Rect;

use rg2d::components::{ Entity, Transform };
use rg2d::physics::Vector2;
use rg2d::graphics::{ render_sprite, Sprite };
use rg2d::content::Context;

mod graphics;
mod physics;
mod components;
mod context;
mod game_loop;


fn main() -> Result<(), String> {
  let (mut ctx, mut event_pump) = context::Context::new();
  ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
  ctx.canvas.clear();
  ctx.canvas.present();

  struct MyGame {
      player: Entity,
      movement_speed: f32,
  }

  impl MyGame {
    pub fn new(mut ctx: &mut context::Context) -> MyGame {
    let path = Path::new("assets/bardo.png");
    let texture = ctx.texture_creator.load_texture(path).unwrap();
      MyGame {
          player: Entity::new(Sprite::new(texture, Rect::new(0, 0, 26, 36)), Transform::default()),
          movement_speed: 2f32
      }
    }
  }
  impl game_loop::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut context::Context) -> Result<(), String> {
      let mut x = 0f32;
      let mut y = 0f32;
      for key in _ctx.input.keys_current.iter() {
        match key {
          Keycode::A => x = -1f32,
          Keycode::D => x = 1f32,
          Keycode::W => y = -1f32,
          Keycode::S => y = 1f32,
          _ => {}
        }
      }
      self.player.transform.position = self.player.transform.position + Vector2::new(x * self.movement_speed, y * self.movement_speed);
      Ok(())
    }

    fn render(&mut self, mut ctx: &mut Context) -> Result<(), String> {
        ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
        ctx.canvas.clear();
        render_sprite(&ctx, &self.player.sprite, self.player.transform.position);
        ctx.canvas.present();
      Ok(())
    }
  }

  let mut my_game = MyGame::new(&mut ctx);

  match game_loop::GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
    Ok(()) => println!("Game exited"),
    Err(e) => println!("Error occured: {}", e),
  }
  Ok(())
}
