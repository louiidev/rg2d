use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use crate::context::Context;
use crate::components::{Sprite, Transform};
use sdl2::pixels::Color;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;


fn get_position_center_to_screen(ctx: &mut Context, position: Point) -> Point {
    let (width, height) = ctx.canvas.output_size().unwrap();
    position + Point::new(width as i32 / 2, height as i32 / 2) - ctx.camera.position
}

fn render_texture(ctx: &mut Context, texture: &Texture, transform: &Transform) {
    let screen_position = get_position_center_to_screen(ctx, transform.position());
    let screen_rect = Rect::from_center(Point::new(screen_position.x as i32 * transform.scale as i32, screen_position.y as i32 * transform.scale as i32), transform.rect.width() * transform.scale, transform.rect.height() * transform.scale);
    ctx.canvas.copy(&texture, transform.rect, screen_rect);
}

pub struct Render {
    
}

impl Render {
    pub fn sprite(ctx: &mut Context, sprite: &Sprite, transform: &Transform) {
        render_texture(ctx, &sprite.texture, &transform);
    }
    pub fn clear(ctx: &mut Context, color: Color) {
        ctx.canvas.set_draw_color(color);
        ctx.canvas.clear();
    }

    pub fn rect(ctx: &mut Context, transform: &Transform, color: Color) {
        ctx.canvas.set_draw_color(color);
        let rect = transform.rect;
        let screen_position = get_position_center_to_screen(ctx, Point::new(rect.x, rect.y));
        let screen_rect = Rect::new(screen_position.x * transform.scale as i32, screen_position.y * transform.scale as i32, rect.width() * transform.scale, rect.height() * transform.scale);
        ctx.canvas.draw_rect(screen_rect);
        ctx.canvas.fill_rect(screen_rect);
    }

    pub fn texture(ctx: &mut Context, texture: &Texture, transform: &Transform) {
        render_texture(ctx, &texture, &transform);
    }

    pub fn text(ctx: &mut Context, font: Font, text: &str, position: Point, color: Color, size: u32) {
        let surface = font.render(&text)
        .blended(color).map_err(|e| e.to_string()).unwrap();
    
        let texture = ctx.texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let padding = 64;
            let target = Rect::new(position.x as i32, position.y as i32, width / 100 * size, height / 100 * size);
        ctx.canvas.copy(&texture, None, Some(target));
    }

    pub fn line(ctx: &mut Context, p1: Point, p2: Point, color: Color) {
        ctx.canvas.set_draw_color(color);
        let pos1 = get_position_center_to_screen(ctx, p1);
        let pos2 = get_position_center_to_screen(ctx, p2);
        ctx.canvas.draw_line(pos1 * 2, pos2 * 2);
    }

}