

use crate::graphics::Sprite;
use crate::physics::Vector2;


pub struct Transform {
  pub position: Vector2
}

impl Transform {
  pub fn default() -> Self {
    Transform {
      position: Vector2::default()
    }
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

