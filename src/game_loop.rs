use sdl2::image::{self, InitFlag};
use sdl2::render::WindowCanvas;
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
  pub fn run<S>(ctx: &mut Context, event_pump: &mut EventPump, state: &mut S) -> Result<(), String> where S: EventHandler {
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        state.update(&mut ctx);
        state.render(&mut ctx);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
  }
}