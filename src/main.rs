use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;

use rg2d::components::{ Transform };
use rg2d::physics::{Vector2};
use rg2d::graphics::{ Render, Sprite };
use rg2d::context::Context;
use rg2d::game_loop::{GameLoop, EventHandler};
use rg2d::collisions::{update_pos_x, update_pos_y};

mod collisions;
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
      walls: Vec<Transform>,
      movement_speed: i32,
  }

  impl MyGame {
    pub fn new(mut _ctx: &mut Context) -> MyGame {
    let sprite_path = Path::new("assets/bardo.png");
    let texture = _ctx.texture_creator.load_texture(sprite_path).unwrap();

    let r1 = Transform::new(-25, -25, 50, 50);

      MyGame {
          r1,
          walls: vec![
            Transform::new(250, 0, 100, 100),
            Transform::new(40, 200, 100, 100),
            Transform::new(500, 32, 100, 100),
            Transform::new(-50, 150, 100, 100),
            Transform::new(-40, -200, 100, 100),
            Transform::new(140, 84, 100, 100),
            Transform::new(-400, 100, 100, 100)
          ],
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

      if x != 0 || y != 0 {
        let mut pot_rect_x = self.r1.rect;
        pot_rect_x.set_x(self.r1.x() + (x * self.movement_speed));
        let mut pot_rect_y = self.r1.rect;
        pot_rect_y.set_y(self.r1.y() + (y * self.movement_speed));
        for wall in self.walls.iter() {
          update_pos_x(&mut pot_rect_x, &wall.rect, x);
          update_pos_y(&mut pot_rect_y, &wall.rect, y);
        }
        self.r1.rect.set_x(pot_rect_x.x());
        self.r1.rect.set_y(pot_rect_y.y());
      }
    }

    fn render(&mut self, mut _ctx: &mut Context) {
        Render::clear(_ctx, Color::RGB(130, 130, 255));
        Render::rect(_ctx, &self.r1, Color::RGB(0, 0, 255));
        for wall in self.walls.iter() {
           Render::rect(_ctx, &wall, Color::RGB(0, 0, 0));
        }
    }
  }

  let mut my_game = MyGame::new(&mut ctx);

  match GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
    Ok(()) => println!("Game exited"),
    Err(e) => println!("Error occured: {}", e),
  }
  Ok(())
}
