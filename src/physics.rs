use sdl2::rect::{Point, Rect};

pub fn center_rect(rect: &Rect) -> Point {
    Point::new(rect.x + (rect.width() as i32 / 2), rect.y + (rect.height() as i32 / 2))
}

pub fn distance_between(a: Point, b: Point) -> f32 {
    let x = (a.x - b.x) as f32;
    let y = (a.y - b.y) as f32;
    (x * x + y * y).sqrt()
}

pub fn lerp(position: i32, target: i32, speed: f32) -> i32 {
    let mut new_pos = (target - position) as f32 * speed;
    new_pos = if target > position {
        new_pos.ceil()
    } else {
        new_pos.floor()
    };
    position + new_pos as i32
}

pub fn lerp_point(position: Point, target: Point, speed: f32) -> Point {
    Point::new(
        lerp(position.x, target.x, speed),
        lerp(position.y, target.y, speed)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lerp() {
        let mut x = 0;
        let target = -64;
        for i in 0..15 {
            x = lerp(x, target, 0.2);
            println!("{}", x);
        }
        assert_eq!(x, target);
    }

}