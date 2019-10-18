use sdl2::image::{self, InitFlag};
use sdl2::render::WindowCanvas;
use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use crate::context::Context;
use std::time::Duration;

pub struct GameLoop {

}

trait Game {
    fn update(&self, ctx: &mut Context);
    fn render(&self, ctx: &mut Context);
}

pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), String>;
    fn render(&mut self, _ctx: &mut Context) -> Result<(), String>;
}


impl GameLoop {
  pub fn run<S>(mut ctx: &mut Context, events: &mut EventPump, state: &mut S) -> Result<(), String> where S: EventHandler {

    'main: loop {
        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'main;
            };
        }
        ctx.input.set_keys(&events);
        state.update(&mut ctx);
        state.render(&mut ctx);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
  }
}