use sdl2::rect::{Point, Rect};

fn center_rect(rect: &Rect) -> Point {
    Point::new(rect.x + (rect.width() as i32 / 2), rect.y + (rect.height() as i32 / 2))
}
