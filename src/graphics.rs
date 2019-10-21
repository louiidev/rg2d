use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use crate::context::Context;
use crate::physics::Vector2;
use sdl2::pixels::Color;

pub struct Sprite {
    pub texture: Texture, // sprite texture
    pub area: Rect // area of sprite to render 
}

impl Sprite {
    pub fn new(texture: Texture, area: Rect) -> Self {
        Sprite {
            texture,
            area
        }
    }
}

fn render_texture(ctx: &mut Context, texture: &Texture, size: Rect, position: Vector2) {
    let (width, height) = ctx.canvas.output_size().unwrap();
    let screen_position = position + Vector2::new(width as f32 / 2f32, height as f32 / 2f32);
    let screen_rect = Rect::from_center(Point::new(screen_position.x as i32, screen_position.y as i32), size.width(), size.height());
    ctx.canvas.copy(&texture, size, screen_rect);
}


pub struct Render {
    
}

impl Render {
    pub fn sprite(ctx: &mut Context, sprite: &Sprite, position: Vector2) {
        render_texture(ctx, &sprite.texture, sprite.area, position);
    }
    pub fn clear(ctx: &mut Context, color: Color) {
        ctx.canvas.set_draw_color(color);
        ctx.canvas.clear();
    }

    pub fn rect(ctx: &mut Context, size: Rect, color: Color) {
        ctx.canvas.set_draw_color(color);
        ctx.canvas.draw_rect(size);
        ctx.canvas.fill_rect(size);
    }

    pub fn texture(ctx: &mut Context, texture: &Texture, size: Rect, position: Vector2) {
        render_texture(ctx, &texture, size, position);
    }

    pub fn text(ctx: &mut Context, text: &str, position: Vector2, color: Color) {
        let surface = ctx.font.render(&text)
        .blended(color).map_err(|e| e.to_string()).unwrap();
    
        let fTexture = ctx.texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();
        render_texture(ctx, &fTexture, Rect::new(20, 20, 250, 30), position);
    }

}