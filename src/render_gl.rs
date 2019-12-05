use gl;
use std;
use std::ffi::{CStr, CString};
use nalgebra_glm::{ Vec2, Mat4 };

pub struct Program {
    pub id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id);
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id);
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn set_int(&self, name: &str, int: i32) {
        let cname = CString::new(name).expect("expected uniform name to have no nul bytes");
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, cname.as_bytes_with_nul().as_ptr() as *const i8),
                int,
            );
        }
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    pub id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub struct gl_entity {
    pub program_id: gl::types::GLuint,
    pub vertex_array_object: gl::types::GLuint,
}

fn render_rect(rect: &gl_entity) {
    unsafe {
        gl::UseProgram(rect.program_id);
        gl::BindVertexArray(rect.vertex_array_object);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        gl::BindVertexArray(0);
    }
}

fn create_rect(program_id: gl::types::GLuint) -> gl_entity {
    let vertices: Vec<f32> = vec![
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
    ];

    let indices = vec![
        // note that we start from 0!
        0, 1, 3, // first Triangle
        1, 2, 3, // second Triangle
    ];

    let mut vertex_buffer_object: gl::types::GLuint = 0;
    let mut vertex_array_object: gl::types::GLuint = 0;
    let mut ebo: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::GenBuffers(1, &mut vertex_buffer_object);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vertex_array_object);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            indices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // position attribute
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    gl_entity {
        vertex_array_object,
        program_id,
    }
}

pub struct Renderer {}

impl Renderer {
    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
    
    pub fn set_projection(shader_id: gl::types::GLuint, width: u32, height: u32, position: Vec2) {
        unsafe {
            gl::UseProgram(shader_id);
        }
        let projection = nalgebra_glm::ortho(position.x, width as f32 + position.x, height as f32 + position.y, position.y, -1.0, 1.0);
        let cname = CString::new("projection").expect("expected uniform name to have no nul bytes");
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(
                    shader_id,
                    cname.as_bytes_with_nul().as_ptr() as *const i8,
                ),
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&projection).as_ptr(),
            );
        }
    }

    pub fn create_rect(program_id: gl::types::GLuint) -> gl_entity {
        create_rect(program_id)
    }
    pub fn rect(rect: &gl_entity, position: Vec2, size: Vec2) {
        unsafe {
            gl::UseProgram(rect.program_id);
            
            let mut model = Mat4::identity();
            model =
                nalgebra_glm::translate(&model, &nalgebra_glm::vec3(position.x, position.y, 0.0));

            model = nalgebra_glm::translate(
                &model,
                &nalgebra_glm::vec3(0.5 * size.x, 0.5 * size.y, 0.0),
            );
            model = nalgebra_glm::rotate(&model, 0.0, &nalgebra_glm::vec3(0.0, 0.0, 1.0)); 
            model = nalgebra_glm::translate(
                &model,
                &nalgebra_glm::vec3(-0.5 * size.x, -0.5 * size.y, 0.0),
            );

            model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(size.x, size.y, 1.0));
            let cname =
                CString::new("model").expect("expected uniform name to have no nul bytes");
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(
                    rect.program_id,
                    cname.as_bytes_with_nul().as_ptr() as *const i8,
                ),
                1,
                gl::FALSE,
                nalgebra_glm::value_ptr(&model).as_ptr(),
            );
            gl::BindVertexArray(rect.vertex_array_object);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
