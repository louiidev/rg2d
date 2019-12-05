use sdl2::event::Event;
use sdl2::EventPump;
use std::time::Duration;
use crate::context::Context;

pub struct GameLoop {}

pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context);
    fn render(&self, _ctx: &mut Context);
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
        ctx.input.set_mouse_state(&events);
        state.update(&mut ctx);
        state.render(&mut ctx);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
  }
}