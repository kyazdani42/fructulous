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
            Self::Julia => 3.0,
        }
    }
}

pub struct Renderer {
    pub generator: Generator,
    pub context: Ctx,
    pub precision: i32,
    pub zoom: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub color_type: i32,
    pub start_time: std::time::Instant,
    pub automate: bool,
    pub fractal_type: FractalType,
    pub n: i32,
}

impl Renderer {
    pub fn new(context: Ctx) -> Self {
        Self {
            generator: Generator::new(),
            context,
            precision: 50,
            zoom: 1.0,
            x_offset: 0.0,
            y_offset: 0.0,
            color_type: 1,
            start_time: std::time::Instant::now(),
            automate: true,
            fractal_type: FractalType::Mandelbrot(1.0),
            n: 3,
        }
    }

    pub fn draw(&mut self) {
        let time = self.start_time.elapsed().as_secs_f32();
        unsafe {
            let generator = &self.generator;
            generator.shader.useProgram();
            generator.shader.setInt("maxIter", self.precision);
            generator.shader.setFloat("zoom", self.zoom);
            generator.shader.setFloat("xOffset", self.x_offset);
            generator.shader.setFloat("yOffset", self.y_offset);
            generator.shader.setInt("colorType", self.color_type);
            generator.shader.setInt("n", self.n);
            generator
                .shader
                .setFloat("algType", self.fractal_type.as_f32());
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
        self.fractal_type = match self.fractal_type {
            FractalType::Julia => FractalType::Mandelbrot(1.0),
            FractalType::Mandelbrot(val) => match val as i32 {
                1 => FractalType::Mandelbrot(2.0),
                2 => FractalType::Julia,
                _ => FractalType::Mandelbrot(1.0),
            },
        }
    }

    pub fn change_n(&mut self, inc: i32) {
        self.n += inc;
        if self.n < 3 {
            self.n = 3;
        }
    }

    pub fn switch_automation(&mut self) {
        self.automate = !self.automate;
    }

    pub fn next_color(&mut self) {
        if self.color_type == 6 {
            self.color_type = 1;
        } else {
            self.color_type += 1;
        }
    }

    pub fn augment_zoom(&mut self) {
        self.zoom += self.zoom;
    }

    pub fn diminish_zoom(&mut self) {
        if self.zoom > 1.0 {
            self.zoom -= self.zoom / 3.0;
        }
    }

    pub fn move_left(&mut self) {
        self.x_offset -= self.zoom / (self.zoom * self.zoom);
    }

    pub fn move_right(&mut self) {
        self.x_offset += self.zoom / (self.zoom * self.zoom);
    }

    pub fn move_up(&mut self) {
        self.y_offset += self.zoom / (self.zoom * self.zoom);
    }

    pub fn move_down(&mut self) {
        self.y_offset -= self.zoom / (self.zoom * self.zoom);
    }

    pub fn diminish_precision(&mut self) {
        if self.precision > 20 {
            self.precision -= 10;
        }
    }

    pub fn augment_precision(&mut self) {
        if self.precision < 5000 {
            self.precision += 10;
        }
    }
}
