use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::path::Path;
use std::time::Duration;
use sdl2::render::TextureCreator;

mod context;
mod game_loop;


fn render_sprite(
  ctx: &mut context::Context,
  texture: &Texture,
  sprite: Rect,
  pos: Point
) -> Result<(), String> {
  let (width, height) = ctx.canvas.output_size()?;
  let screen_position = pos + Point::new(width as i32 / 2, height as i32 / 2);
  let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());

  ctx.canvas.copy(texture, sprite, screen_rect)?;
  Ok(())
}

fn main() -> Result<(), String> {
  let (mut ctx, mut event_pump) = context::Context::new();
  ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
  ctx.canvas.clear();
  ctx.canvas.present();

  struct MyGame {
      texture: Texture,
      position: Point
  }

  impl MyGame {
    pub fn new(mut ctx: &mut context::Context) -> MyGame {
    let path = Path::new("assets/bardo.png");
    let texture = ctx.texture_creator.load_texture(path).unwrap();
      MyGame {
          texture,
          position: Point::new(0, 0)
      }
    }
  }
  impl game_loop::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut context::Context) -> Result<(), String> {
      self.position = self.position.offset(1, 0);
      Ok(())
    }

    fn render(&mut self, mut ctx: &mut context::Context) -> Result<(), String> {
        ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
        ctx.canvas.clear();
        render_sprite(&mut ctx, &self.texture, Rect::new(0, 0, 26, 36), self.position);
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
