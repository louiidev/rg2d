use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;

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

    pub fn right() -> Self {
        Vector2 {
            x: 1,
            y: 0
        }
    }

    pub fn left() -> Self {
        Vector2 {
            x: -1,
            y: 0
        }
    }

    pub fn up() -> Self {
        Vector2 {
            x: 0,
            y: 1
        }
    }

    pub fn down() -> Self {
        Vector2 {
            x: 0,
            y: -1
        }
    }


    pub fn multipleBy(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other
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

