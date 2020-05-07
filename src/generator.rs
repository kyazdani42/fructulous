#![allow(non_upper_case_globals)]
use std::ffi::c_void;
use std::mem;

use super::Shader;

use gl::types::*;

static vertex_shader_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/vertex.glsl");
static fragment_shader_path: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/fragment.glsl");

pub struct Generator {
    pub VAO: u32,
    pub num_vertices: i32,
    pub shader: Shader,
}

impl Generator {
    pub fn new() -> Self {
        let (VAO, num_vertices) = unsafe {
            let (mut VAO, mut VBO) = (0, 0);
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);

            gl::BindVertexArray(VAO);

            let vertices: [f32; 12] = [
                -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0,
            ];

            let float_size = mem::size_of::<GLfloat>();
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * float_size) as GLsizeiptr,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            let stride = 2 * float_size as GLsizei;
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            (VAO, vertices.len())
        };

        Self {
            VAO,
            num_vertices: num_vertices as i32,
            shader: Shader::new(vertex_shader_path, fragment_shader_path),
        }
    }
}
