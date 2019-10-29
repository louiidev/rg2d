use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;
use sdl2::rect::{Point, Rect};

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2 {
            x,
            y
        }
    }

    pub fn default() -> Self {
        Vector2::new(0, 0)
    }

    pub fn right() -> Self {
        Vector2::new(1, 0)
    }

    pub fn left() -> Self {
        Vector2::new(-1, 0)
    }

    pub fn up() -> Self {
        Vector2::new(0, 1)
    }

    pub fn down() -> Self {
        Vector2::new(0, -1)
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

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
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

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

fn center_rect(rect: &Rect) -> Point {
    Point::new(rect.x + (rect.width() as i32 / 2), rect.y + (rect.height() as i32 / 2))
}
