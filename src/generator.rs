#![allow(non_upper_case_globals)]
use std::ffi::c_void;
use std::mem;

use super::{Shader, HEIGHT, WIDTH};

use gl::types::*;

static gpu_vtx_shad: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/gpu_vert.glsl");
static gpu_frag_shad: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/gpu_frag.glsl");

static cpu_frag_shad: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/cpu_frag.glsl");
static cpu_vtx_shad: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/res/cpu_vert.glsl");

pub enum ComputeType {
    Gpu,
    Cpu,
}

pub struct Generator {
    pub VAO: u32,
    pub num_vertices: i32,
    pub shader: Shader,
}

impl Generator {
    pub fn new(compute_type: ComputeType) -> Self {
        match compute_type {
            ComputeType::Gpu => gen_gpu(),
            ComputeType::Cpu => gen_cpu(),
        }
    }
}

fn gen_gpu() -> Generator {
    let (VAO, num_vertices) = unsafe {
        let (mut VAO, mut VBO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(VAO);

        let vertices = create_vertices();

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

    Generator {
        VAO,
        num_vertices: num_vertices as i32,
        shader: Shader::new(gpu_vtx_shad, gpu_frag_shad),
    }
}

fn create_vertices() -> Vec<f32> {
    let mut vertices: Vec<f32> = vec![0.0; (HEIGHT * WIDTH) as usize * 2];

    let half_height = HEIGHT / 2;
    let half_width = WIDTH / 2;
    for y in 0..HEIGHT {
        let y_pos = (y as f32 / half_height as f32) - 1.0;
        for x in 0..WIDTH {
            let x_pos = (x as f32 / half_width as f32) - 1.0;
            let idx = (y * WIDTH + x) as usize * 2;
            vertices[idx] = x_pos;
            vertices[idx + 1] = y_pos;
        }
    }

    vertices
}

fn gen_cpu() -> Generator {
    let (VAO, num_vertices) = unsafe {
        let (mut VAO, mut VBO) = (0, 0);
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);

        gl::BindVertexArray(VAO);

        let vertices = create_mandelbrot_vertices();

        let float_size = mem::size_of::<GLfloat>();
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * float_size) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        let stride = 5 * float_size as GLsizei;

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (2 * float_size) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        (VAO, vertices.len())
    };

    Generator {
        VAO,
        num_vertices: num_vertices as i32,
        shader: Shader::new(cpu_vtx_shad, cpu_frag_shad),
    }
}

pub fn create_mandelbrot_vertices() -> Vec<f32> {
    vec![]
}
