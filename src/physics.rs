use sdl2::rect::{Point, Rect};

fn center_rect(rect: &Rect) -> Point {
    Point::new(rect.x + (rect.width() as i32 / 2), rect.y + (rect.height() as i32 / 2))
}

pub fn distance_between(a: Point, b: Point) -> f32 {
    let x = (a.x - b.x) as f32;
    let y = (a.y - b.y) as f32;
    (x * x + y * y).sqrt()
}