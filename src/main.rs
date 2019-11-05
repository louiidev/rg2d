// use sdl2::image::LoadTexture;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use std::path::Path;
// use sdl2::rect::{Rect, Point};
use sdl2::image::LoadTexture;
use rg2d::components::{ Transform, Sprite };
use rg2d::graphics::{ Render };
use rg2d::context::{ResourceManager, Context};
use rg2d::game_loop::{GameLoop, EventHandler};
use rg2d::collisions::{update_pos_x, update_pos_y, raycast};

// mod collisions;
// mod graphics;
// mod physics;
// mod components;
// mod context;
// mod game_loop;

// const GRAVITY: i32 = 4;
// const MOVEMENT_SPEED: i32 = 6;


// fn handle_movement(rect: &mut Rect, velocity: Point) -> (Rect, Rect) {
//       let mut pot_rect_x = Rect::new(rect.x, rect.y, rect.width(), rect.height());
//       pot_rect_x.set_x(pot_rect_x.x() + (velocity.x * MOVEMENT_SPEED));
//       let mut pot_rect_y =  Rect::new(rect.x, rect.y, rect.width(), rect.height());
//       let y = if velocity.y != 0 {
//         velocity.y
//       } else {
//         GRAVITY
//       };
//       pot_rect_y.set_y(pot_rect_x.y() + y);
//       (pot_rect_x, pot_rect_y)
// }



// fn main() -> Result<(), String> {
//   let (mut ctx, mut event_pump) = Context::new();
//   ctx.canvas.set_draw_color(Color::RGB(130, 130, 255));
//   ctx.canvas.clear();
//   ctx.canvas.present();

//   struct MyGame {
//       player: Transform,
//       walls: Vec<Transform>,
//   }

//   impl MyGame {
//     pub fn new(mut _ctx: &mut Context) -> MyGame {
//     let sprite_path = Path::new("assets/bardo.png");
//     let texture = _ctx.texture_creator.load_texture(sprite_path).unwrap();

//     let player = Transform::new(-25, -25, 50, 50);

//       MyGame {
//           player,
//           walls: vec![
//             Transform::new(250, 0, 50, 50),
//             Transform::new(40, 200, 50, 50),
//             Transform::new(500, 32, 50, 50),
//             Transform::new(-50, 150, 50, 50),
//             Transform::new(-40, -200, 50, 50),
//             Transform::new(140, 84, 50, 50),
//             Transform::new(-400, 50, 50, 50),
//             Transform::new(-600, -400, 20, 800),
//             Transform::new(580, -400, 20, 800),
//             Transform::new(-600, -400, 1200, 20),
//             Transform::new(-580, 380, 1200, 20)
//           ],
//       }
//     }
//   }

//   impl EventHandler for MyGame {
//     fn update(&mut self, _ctx: &mut Context) {
//       let mut velocity = Point::new(0, GRAVITY);
//       for key in _ctx.input.keys_current.iter() {
//         match key {
//           Keycode::A | Keycode::Left => velocity.x = -1,
//           Keycode::D | Keycode::Right => velocity.x = 1,
//           Keycode::Up | Keycode::Space => velocity.y = -10,
//           _ => {}
//         }
//       }
//         let (mut pot_rect_x, mut pot_rect_y) = handle_movement(&mut self.player.rect, velocity);
//         // example of collision handling
//         for wall in self.walls.iter() {
//           update_pos_x(&mut pot_rect_x, &wall.rect, velocity.x);
//           update_pos_y(&mut pot_rect_y, &wall.rect, velocity.y);
//         }
//         self.player.rect.set_x(pot_rect_x.x());
//         self.player.rect.set_y(pot_rect_y.y());

        
//     }

//     fn render(&mut self, mut _ctx: &mut Context) {
//         Render::clear(_ctx, Color::RGB(130, 130, 255));
//         Render::rect(_ctx, &self.player, Color::RGB(0, 0, 255));
//         for wall in self.walls.iter() {
//            Render::rect(_ctx, &wall, Color::RGB(0, 0, 0));
//         }
//     }
//   }

//   let mut my_game = MyGame::new(&mut ctx);

//   match GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
//     Ok(()) => println!("Game exited"),
//     Err(e) => println!("Error occured: {}", e),
//   }
//   Ok(())
// }


fn main() {
  let (mut ctx, mut event_pump) = Context::new(None);
  let texture_creator = ctx.canvas.texture_creator();
  let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
  let resourceManager = ResourceManager::new(&texture_creator, &ttf_context);

    struct MyGame <'t> {
        resourceManager: ResourceManager<'t>
    }

    impl <'t> EventHandler for MyGame <'t> {
        fn update(&mut self, _ctx: &mut Context) {
            // self.resourceManager.textures.
        }

        fn render(&self, _ctx: &mut Context) {

        }
    }

    let mut my_game = MyGame {
        resourceManager
    };

    match GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
        Ok(()) => println!("Game exited"),
        Err(e) => println!("Error occured: {}", e),
    }
}