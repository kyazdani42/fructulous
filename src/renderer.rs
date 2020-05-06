use super::{Ctx, Generator, ComputeType};

pub struct Renderer {
    pub gentype: ComputeType,
    pub gpu_generator: Generator,
    pub cpu_generator: Generator,
    pub context: Ctx,
    pub precision: i32,
    pub zoom: f32,
    pub xOffset: f32,
    pub yOffset: f32,
}

impl Renderer {
    pub fn new(gpu_generator: Generator, cpu_generator: Generator, context: Ctx) -> Self {
        Self {
            gentype: ComputeType::Gpu,
            gpu_generator,
            cpu_generator,
            context,
            precision: 50,
            zoom: 1.0,
            xOffset: 0.0,
            yOffset: 0.0,
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            let generator = self.get_generator();
            generator.shader.useProgram();
            generator.shader.setInt("maxIter", self.precision);
            generator.shader.setFloat("zoom", self.zoom);
            generator.shader.setFloat("xOffset", self.xOffset);
            generator.shader.setFloat("yOffset", self.yOffset);

            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(generator.VAO);
            gl::DrawArrays(gl::POINTS, 0, generator.num_vertices);
            gl::BindVertexArray(0);
        }

        self.context.swap_buffers().expect("swap buffers");
    }

    pub fn get_generator(&self) -> &Generator {
        match self.gentype {
            ComputeType::Gpu => &self.gpu_generator,
            ComputeType::Cpu => &self.cpu_generator,
        }
    }

    pub fn switch_generator(&mut self) {
        match self.gentype {
            ComputeType::Cpu => self.gentype = ComputeType::Gpu,
            ComputeType::Gpu => self.gentype = ComputeType::Cpu,
        }
    }

    pub fn augment_zoom(&mut self) {
        self.zoom += self.zoom;
    }

    pub fn diminish_zoom(&mut self) {
        if self.zoom <= 1.0 {
            return;
        }
        self.zoom -= self.zoom / 2.0;
    }

    pub fn move_left(&mut self) {
        self.xOffset -= 0.1 / self.zoom;
    }

    pub fn move_right(&mut self) {
        self.xOffset += 0.1 / self.zoom;
    }

    pub fn move_up(&mut self) {
        self.yOffset += 0.1 / self.zoom;
    }

    pub fn move_down(&mut self) {
        self.yOffset -= 0.1 / self.zoom;
    }

    pub fn diminish_precision(&mut self) {
        if self.precision == 20 {
            return;
        }
        self.precision -= 10;
    }

    pub fn augment_precision(&mut self) {
        if self.precision == 1000 {
            return;
        }
        self.precision += 10;
    }
}
