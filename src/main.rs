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

struct Sprite {
    texture: Texture, // sprite texture
    area: Rect // area of sprite to render 
}

impl Sprite {
    fn new(texture: Texture, area: Rect) -> Self {
        Sprite {
            texture,
            area
        }
    }
}

struct Entity {
    sprite: Sprite,
    position: Point,
}

impl Entity {
    fn new(sprite: Sprite, position: Point) -> Self {
        Entity {
            sprite,
            position
        }
    }
}


fn render_sprite(
  ctx: &mut context::Context,
  entity: &Entity
) -> Result<(), String> {
  let sprite = &entity.sprite;
  let (width, height) = ctx.canvas.output_size()?;
  let screen_position = entity.position + Point::new(width as i32 / 2, height as i32 / 2);
  let screen_rect = Rect::from_center(screen_position, sprite.area.width(), sprite.area.height());

  ctx.canvas.copy(&sprite.texture, sprite.area, screen_rect)?;
  Ok(())
}

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
          player: Entity::new(Sprite::new(texture, Rect::new(0, 0, 26, 36)), Point::new(0, 0)),
          movement_speed: 2f32
      }
    }
  }
  impl game_loop::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut context::Context) -> Result<(), String> {
      let mut x = 0;
      let mut y = 0;
      for key in _ctx.input.keys_current.iter() {
        match key {
          Keycode::A => x = -1,
          Keycode::D => x = 1,
          Keycode::W => y = -1,
          Keycode::S => y = 1,
          _ => {}
        }
      }
      self.player.position = self.player.position.offset(x * self.movement_speed, y * self.movement_speed);
      Ok(())
    }

    fn render(&mut self, mut ctx: &mut context::Context) -> Result<(), String> {
        ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
        ctx.canvas.clear();
        render_sprite(&mut ctx, &self.player);
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
