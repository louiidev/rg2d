use sdl2::render::Texture;
use sdl2::rect::{Point, Rect};

pub struct Sprite {
  pub texture: Texture
}

pub struct Transform {
  pub position: Point,
  pub size: Rect,
  pub scale: u32,
}

impl Transform {
  pub fn new(size: Rect, position: Point) -> Self {
    Transform {
      size,
      position,
      scale: 2
    }
  }
}

