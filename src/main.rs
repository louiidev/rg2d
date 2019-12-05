use image::imageops::{ flip_vertical};
use image::{DynamicImage, RgbaImage};
use rg2d::context::{ Context};
use rg2d::game_loop::{EventHandler, GameLoop};
use rg2d::render_gl::{Program, Shader, Renderer, gl_entity};
use std::ffi::c_void;
use std::ffi::{CString};

fn build_opengl_mipmapped_texture(image: RgbaImage) -> gl::types::GLuint {
    unsafe {
        let mut texture_id: gl::types::GLuint = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        let width = image.width();
        let height = image.height();
        let img_data = image.into_raw();
        let img_ptr = img_data.as_ptr() as *const c_void;
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img_ptr,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // gl::BindTexture(gl::TEXTURE_2D, 0);

        texture_id
    }
}

fn load_image(file: String) -> Result<RgbaImage, String> {
    match image::open(file.clone()) {
        Err(err) => panic!("Could not load image {}: {}", file, err),
        Ok(img) => {
            let img = match img {
                DynamicImage::ImageRgba8(img) => img,
                img => img.to_rgba(),
            };
            Ok(flip_vertical(&img))
        }
    }
}

fn main() {
    let (mut ctx, mut event_pump) = Context::new(None);
    struct MyGame {
        rect: gl_entity,
        texture_id: gl::types::GLuint,
        direction: nalgebra_glm::Vec2,
        position: nalgebra_glm::Vec2
    }
    
    let vert_shader = Shader::from_vert_source(
        &CString::new(include_str!("../shaders/rect.vert")).unwrap(),
    )
    .unwrap();

    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("../shaders/rect.frag")).unwrap(),
    )
    .unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    let rect = Renderer::create_rect(shader_program.id);
    let (width, height) = ctx.window.size();
    Renderer::set_projection(shader_program.id, width, height);
    Renderer::set_view(shader_program.id);

    impl EventHandler for MyGame {
        fn update(&mut self, ctx: &mut Context) {
            let (width, _height) = ctx.window.size();
            if self.position.x > width as f32 - 25.0 ||  self.position.x < 25.0 {
                self.direction = -self.direction;
            }

            self.position+= self.direction;
        }

        fn render(&self, ctx: &mut Context) {
            Renderer::clear();
            Renderer::rect(&self.rect, self.position + self.direction, nalgebra_glm::vec2(50.0, 50.0));
            ctx.window.gl_swap_window();
        }
    }
    let mut my_game = MyGame {
        rect,
        texture_id: 0,
        position: nalgebra_glm::vec2(25.0, 400.0),
        direction: nalgebra_glm::vec2(5.0, 0.0)
    };


    
    

    Renderer::clear_color(0.3, 0.3, 0.5, 1.0);

    match GameLoop::run(&mut ctx, &mut event_pump, &mut my_game) {
        Ok(()) => println!("Game exited"),
        Err(e) => println!("Error occured: {}", e),
    }
}
