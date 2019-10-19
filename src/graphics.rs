use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use crate::context::Context;
use crate::physics::Vector2;


pub struct Sprite {
    pub texture: Texture, // sprite texture
    pub area: Rect // area of sprite to render 
}

impl Sprite {
    pub fn new(texture: Texture, area: Rect) -> Self {
        Sprite {
            texture,
            area
        }
    }
}


pub fn render_sprite(
  ctx: &mut Context,
  sprite: &Sprite,
  position: Vector2
) -> Result<(), String> {
  let (width, height) = ctx.canvas.output_size()?;
  let screen_position = position + Vector2::new(width as f32 / 2f32, height as f32 / 2f32);
  let screen_rect = Rect::from_center(Point::new(screen_position.x as i32, screen_position.y as i32), sprite.area.width(), sprite.area.height());

  ctx.canvas.copy(&sprite.texture, sprite.area, screen_rect)?;
  Ok(())
}