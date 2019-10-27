use crate::physics::Vector2;
use crate::components::Transform;
use sdl2::rect::Rect;



pub fn update_pos_x(r1: &mut Rect, r2: &Rect, x: i32) {
    if x != 0 && intersect(r1, r2) {
        if x > 0 {
            r1.set_right(r2.x);
        } else {
            r1.set_x(r2.right());
        }
    }
}

pub fn update_pos_y(r1: &mut Rect, r2: &Rect, y: i32) {
    if y != 0 && intersect(r1, r2) {
        if y > 0 {
            r1.set_bottom(r2.top());
        } else {
            r1.set_y(r2.bottom());
        }
    }
}

fn intersect(base: &Rect, target: &Rect) -> bool {
    if base.right() <= target.x || base.x >= target.right() { return false }
    if base.bottom() <= target.top() || base.top() >= target.bottom() { return false }
    true
}