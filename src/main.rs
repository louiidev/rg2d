extern crate gl;
extern crate sdl2;

use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, WindowCanvas, Texture};
use std::time::Duration;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Point, Rect};


const PLAYER_MOVEMENT_SPEED: i32 = 20;

enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
}

fn render_sprite(canvas: &mut WindowCanvas, texture: &Texture, player: &Player) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(texture, player.sprite, screen_rect)?;
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem.window("rg2d", 800, 600)
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    
    let texture_creator = canvas.texture_creator();
    let path = Path::new("assets/bardo.png");
    let texture = texture_creator.load_texture(path).unwrap();

    let mut player_data = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 5,
        direction: Direction::Right
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main,
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    player_data.position = player_data.position.offset(-player_data.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    player_data.position = player_data.position.offset(player_data.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    player_data.position =  player_data.position.offset(0, -player_data.speed);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    player_data.position = player_data.position.offset(0, player_data.speed);
                },
                _ => {}
            }
        }

            canvas.set_draw_color(Color::RGB(130, 130, 255));
    canvas.clear();
        render_sprite(&mut canvas, &texture, &Player {
            position: Point::new(-50, 50),
            sprite: Rect::new(0, 0, 26, 36),
            speed: 5,
            direction: Direction::Right
        });
        render_sprite(&mut canvas, &texture, &player_data);
        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
