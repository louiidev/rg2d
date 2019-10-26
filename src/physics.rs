use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::AddAssign;
use sdl2::rect::Rect;

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

    pub fn multiple_by(self, other: i32) -> Self {
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

pub fn distance_between(v1: Vector2, v2: Vector2) {
    let x = v1.x - v2.x;
    let y = v1.y - v2.y;

    // sqrt(x * x +  y * y)
}

pub fn intersect(base: Rect, target: Rect) -> bool {
    if base.right() <= target.x || base.x >= target.right() { return false }
    if base.bottom() <= target.top() || base.top() >= target.bottom() { return false }
    true
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_intersect() {
        let r1 = Rect::new(0, 5, 5, 5);
        let r2 = Rect::new(-5, 5, 5, 5);
        assert_eq!(intersect(r1, r2), false);
    }
}