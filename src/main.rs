extern crate sdl2;
extern crate gl;


fn setup() {
  
}

fn gameloop() {
  
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("rg2d", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    'main: loop {
        unsafe {
          gl::Viewport(0, 0, 900, 700);
          gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
          gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
        // render window contents here
    }
}
