use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::ttf::Font;
use std::path::Path;

use rg2d::components::{ Entity, Transform };
use rg2d::physics::Vector2;
use rg2d::graphics::{ Render, Sprite };
use rg2d::context::Context;

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
      movement_speed: f32,
      font: Texture,
  }

  impl MyGame {
    pub fn new(mut _ctx: &mut Context) -> MyGame {
    let sprite_path = Path::new("assets/bardo.png");
    let font_path = Path::new("fonts/small_pixel.ttf");
    let texture = _ctx.texture_creator.load_texture(sprite_path).unwrap();
    let mut font = _ctx.tff.load_font(font_path, 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);
  
    let surface = font.render("Hello Rust!")
        .blended(Color::RGBA(0, 0, 0, 0)).map_err(|e| e.to_string()).unwrap();
    
    let fTexture = _ctx.texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string()).unwrap();
      MyGame {
          player: Entity::new(Sprite::new(texture, Rect::new(0, 0, 26, 36)), Transform::default()),
          movement_speed: 2f32,
          font: fTexture,
      }
    }
  }
  impl game_loop::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), String> {
      let mut x = 0f32;
      let mut y = 0f32;
      for key in _ctx.input.keys_current.iter() {
        match key {
          Keycode::A | Keycode::Left => x = -1f32,
          Keycode::D | Keycode::Right => x = 1f32,
          Keycode::W | Keycode::Up => y = -1f32,
          Keycode::S | Keycode::Down => y = 1f32,
          _ => {}
        }
      }
      self.player.transform.position = self.player.transform.position + Vector2::new(x * self.movement_speed, y * self.movement_speed);
      Ok(())
    }

    fn render(&mut self, mut _ctx: &mut Context) -> Result<(), String> {
        Render::clear(_ctx, Color::RGB(130, 130, 255));
        Render::sprite(_ctx, &self.player.sprite, self.player.transform.position);
        _ctx.canvas.copy(&self.font, None, Rect::new(20, 20, 250, 30))?;
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
