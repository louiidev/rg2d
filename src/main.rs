use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::path::Path;

use rg2d::components::{ Entity, Transform };
use rg2d::physics::{intersect, Vector2};
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
      r1: Transform,
      r2: Transform,
      movement_speed: i32,
  }

  impl MyGame {
    pub fn new(mut _ctx: &mut Context) -> MyGame {
    let sprite_path = Path::new("assets/bardo.png");
    let texture = _ctx.texture_creator.load_texture(sprite_path).unwrap();

    let r1 = Transform::new(-25, -25, 50, 50);
    let r2 = Transform::new(250, 0, 50, 50);

      MyGame {
          r1,
          r2,
          movement_speed: 5,
      }
    }
  }

  impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) {
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

      if x != 0 {
        let mut r1 = self.r1.rect;
        r1.set_x(r1.x + x * self.movement_speed);
        if !intersect(r1, self.r2.rect) {
          self.r1.rect.set_x(r1.x);
        } else if x > 0 {
            self.r1.rect.set_right(self.r2.x());
        } else {
          self.r1.rect.set_x(self.r2.rect.right());
        }
      }

      if y != 0 {
        let mut r1 = self.r1.rect;
        r1.set_y(r1.y + y * self.movement_speed);
        if !intersect(r1, self.r2.rect) {
          self.r1.rect.set_y(r1.y);
        } else if y < 0 {
          self.r1.rect.set_y(self.r2.rect.bottom());
        } else {
          self.r1.rect.set_bottom(self.r2.rect.top());
        }

      }
      
    }

    fn render(&mut self, mut _ctx: &mut Context) {
        Render::clear(_ctx, Color::RGB(130, 130, 255));
        Render::rect(_ctx, &self.r1, Color::RGB(0, 0, 255));
        Render::rect(_ctx, &self.r2, Color::RGB(0, 0, 0));
    }
  }

  let mut my_game = MyGame::new(&mut ctx);

  match GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
    Ok(()) => println!("Game exited"),
    Err(e) => println!("Error occured: {}", e),
  }
  Ok(())
}
