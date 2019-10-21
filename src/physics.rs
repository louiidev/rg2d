use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn default() -> Self {
        Vector2 {
            x: 0,
            y: 0
        }
    }

    pub fn new(x: i32, y: i32) -> Self {
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