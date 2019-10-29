use crate::physics::Vector2;
use crate::components::Transform;
use sdl2::rect::{Point, Rect};



pub fn update_pos_x(r1: &mut Rect, r2: &Rect, x: i32) {
    if x != 0 && r1.has_intersection(*r2) {
        if x > 0 {
            r1.set_right(r2.x);
        } else {
            r1.set_x(r2.right());
        }
    }
}

pub fn update_pos_y(r1: &mut Rect, r2: &Rect, y: i32) {
    if y != 0 && r1.has_intersection(*r2) {
        if y > 0 {
            r1.set_bottom(r2.top());
        } else {
            r1.set_y(r2.bottom());
        }
    }
}

fn line_intersect(a: Point, b: Point, c: Point, d: Point) -> Option<Point> {
    let r = b - a;
	let s = d - c;
	let e = r.x * s.y - r.y * s.x; 
	let u = ((c.x - a.x) * r.y - (c.y - a.y) * r.x) / e;
	let t = ((c.x - a.x) * s.y - (c.y - a.y) * s.x) / e;
	if 0 <= u && u <= 1 && 0 <= t && t <= 1 {
        return Some(a + (r * t));
    }
    None
}


pub fn raycast(to: Point, from: Point, query: &Rect) -> Option<Point> {
    match query.intersect_line(to, from) {
        Some(hit) => Some(hit.0),
        None => None
    }
}