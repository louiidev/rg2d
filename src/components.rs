

use crate::graphics::Sprite;
use crate::physics::Vector2;
use sdl2::rect::Rect;

pub struct Transform {
  pub rect: Rect,
  pub scale: u32,
}

impl Transform {
  pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
    Transform {
      rect: Rect::new(x, y, width as u32, height as u32),
      scale: 2
    }
  }

  pub fn position(&self) -> Vector2 {
    Vector2::new(self.rect.x, self.rect.y)
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

pub struct Entity {
    pub sprite: Sprite,
    pub transform: Transform,
}

impl Entity {
    pub fn new(sprite: Sprite, transform: Transform) -> Self {
        Entity {
            sprite,
            transform
        }
    }
}

