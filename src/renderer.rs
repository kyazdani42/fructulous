use super::generator::Generator;
use super::Ctx;

pub enum FractalType {
    Mandelbrot(f32),
    Julia,
}

impl FractalType {
    pub fn as_f32(&self) -> f32 {
        match self {
            Self::Mandelbrot(v) => *v,
            Self::Julia => 3.0
        }
    }
}

pub struct Renderer {
    pub generator: Generator,
    pub context: Ctx,
    pub precision: i32,
    pub zoom: f32,
    pub xOffset: f32,
    pub yOffset: f32,
    pub colorType: i32,
    pub start_time: std::time::Instant,
    pub automate: bool,
    pub fractalType: FractalType,
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
            colorType: 1,
            start_time: std::time::Instant::now(),
            automate: true,
            fractalType: FractalType::Mandelbrot(2.0),
        }
    }

    pub fn draw(&mut self) {
        let time = self.start_time.elapsed().as_secs_f32();
        unsafe {
            let generator = &self.generator;
            generator.shader.useProgram();
            generator.shader.setInt("maxIter", self.precision);
            generator.shader.setFloat("zoom", self.zoom);
            generator.shader.setFloat("xOffset", self.xOffset);
            generator.shader.setFloat("yOffset", self.yOffset);
            generator.shader.setInt("colorType", self.colorType);
            generator.shader.setFloat("algType", self.fractalType.as_f32());
            if self.automate {
                generator.shader.setFloat("time", time);
            } else {
                generator.shader.setFloat("time", 1.0);
            }

            gl::BindVertexArray(generator.VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, generator.num_vertices);
            gl::BindVertexArray(0);
        }

        self.context.swap_buffers().expect("swap buffers");
    }

    pub fn next_fractal_type(&mut self) {
        self.fractalType = match self.fractalType {
            FractalType::Julia => FractalType::Mandelbrot(1.0),
            FractalType::Mandelbrot(val) => match val as i32 {
                1 => FractalType::Mandelbrot(2.0),
                2 => FractalType::Julia,
                _ => FractalType::Mandelbrot(1.0),
            },
        }
    }

    pub fn switch_automation(&mut self) {
        self.automate = !self.automate;
    }

    pub fn next_color(&mut self) {
        if self.colorType == 6 {
            self.colorType = 1;
        } else {
            self.colorType += 1;
        }
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
