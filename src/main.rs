
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::time::Duration;
use sdl2::pixels::Color;
mod context;
mod game_loop;


fn main() {
   let (mut ctx, mut event_pump) = context::Context::new();
    ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
    ctx.canvas.clear();
    ctx.canvas.present();
    

    struct MyGame {
    // Your state here...
        }

        impl MyGame {
            pub fn new(_ctx: &mut context::Context) -> MyGame {
                // Load/create resources here: images, fonts, sounds, etc.
                MyGame { }
            }
        }
        impl game_loop::EventHandler for MyGame {
            fn update(&mut self, _ctx: &mut context::Context) -> Result<(), String> {
                // Update code here...
                println!("updates");
                Ok(())
            }

            fn render(&mut self, ctx: &mut context::Context) -> Result<(), String> {
               Ok(())
            }
        }

        let mut my_game = MyGame::new(&mut ctx);

    match game_loop::GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
        Ok(()) => println!("Game exited"),
        Err(e) => println!("Error occured: {}", e)
    }

}