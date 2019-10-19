use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn default() -> Self {
        Vector2 {
            x: 0f32,
            y: 0f32
        }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {
            x,
            y
        }
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}