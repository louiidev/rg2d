

use sdl2::render::Texture;
use sdl2::rect::{Point, Rect};

pub struct Sprite <'t> {
  pub texture: Texture<'t>
}

pub struct Transform {
  pub position: Point,
  pub rect: Rect,
  pub scale: u32,
}

impl Transform {
  pub fn new(rect: Rect, position: Point) -> Self {
    Transform {
      rect,
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

