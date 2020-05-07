use super::generator::Generator;
use super::Ctx;

pub struct Renderer {
    pub generator: Generator,
    pub context: Ctx,
    pub precision: i32,
    pub zoom: f32,
    pub xOffset: f32,
    pub yOffset: f32,
}

impl Renderer {
    pub fn new(context: Ctx) -> Self {
        Self {
            generator: Generator::new(),
            context,
            precision: 50,
            zoom: 1.0,
            xOffset: 0.0,
            yOffset: 0.0,
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            let generator = &self.generator;
            generator.shader.useProgram();
            generator.shader.setInt("maxIter", self.precision);
            generator.shader.setFloat("zoom", self.zoom);
            generator.shader.setFloat("xOffset", self.xOffset);
            generator.shader.setFloat("yOffset", self.yOffset);

            gl::BindVertexArray(generator.VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, generator.num_vertices);
            gl::BindVertexArray(0);
        }

        self.context.swap_buffers().expect("swap buffers");
    }

    pub fn augment_zoom(&mut self) {
        self.zoom += self.zoom;
    }

    pub fn diminish_zoom(&mut self) {
        if self.zoom <= 1.0 {
            return;
        }
        self.zoom -= self.zoom / 3.0;
    }

    pub fn move_left(&mut self) {
        self.xOffset -= self.zoom / (self.zoom * self.zoom);
    }

    pub fn move_right(&mut self) {
        self.xOffset += self.zoom / (self.zoom * self.zoom);
    }

    pub fn move_up(&mut self) {
        self.yOffset += self.zoom / (self.zoom * self.zoom);
    }

    pub fn move_down(&mut self) {
        self.yOffset -= self.zoom / (self.zoom * self.zoom);
    }

    pub fn diminish_precision(&mut self) {
        if self.precision == 20 {
            return;
        }
        self.precision -= 10;
    }

    pub fn augment_precision(&mut self) {
        if self.precision == 5000 {
            return;
        }
        self.precision += 10;
    }
}
