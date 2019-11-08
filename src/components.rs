

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

  pub fn x(&self) -> i32 {
    self.rect.x
  }

  pub fn y(&self) -> i32 {
    self.rect.y
  }

  pub fn width(&self) -> u32 {
   self.rect.width()
  }

  pub fn height(&self) -> u32 {
   self.rect.height()
  }

}

