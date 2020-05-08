#![allow(non_snake_case)]
use glutin::dpi::PhysicalSize;
use glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::Window;
use glutin::window::WindowBuilder;
use glutin::Api;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::GlRequest;
use glutin::PossiblyCurrent;

mod shader;
use shader::Shader;

mod renderer;
use renderer::Renderer;

mod generator;

pub const WIDTH: i32 = 1920;
pub const HEIGHT: i32 = 1080;

pub type Ctx = ContextWrapper<PossiblyCurrent, Window>;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("dev")
        .with_visible(true)
        .with_resizable(true)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT));

    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (4, 4)))
        .build_windowed(wb, &el);

    let context = match context {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let context = unsafe { context.make_current().expect("Make context current") };
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
    unsafe { gl::Viewport(0, 0, WIDTH, HEIGHT) }

    let mut renderer = Renderer::new(context);

    let frame_time = 1000 / 60;
    let mut timer = std::time::Instant::now();
    el.run(move |event, _, control_flow| {
        if timer.elapsed().as_millis() > frame_time {
            renderer.draw();
            timer = std::time::Instant::now();
        }
        *control_flow = match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    renderer.context.resize(size);
                    let rounded = size.cast::<i32>();
                    unsafe { gl::Viewport(0, 0, rounded.width, rounded.height) }
                    ControlFlow::Poll
                }
                WindowEvent::KeyboardInput { input, .. } => handle_keycodes(input, &mut renderer),
                _ => ControlFlow::Poll,
            },
            Event::RedrawRequested(_) => {
                renderer.draw();
                ControlFlow::Poll
            }
            _ => ControlFlow::Poll,
        };
    });
}

fn handle_keycodes(input: KeyboardInput, renderer: &mut Renderer) -> ControlFlow {
    if let glutin::event::ElementState::Released = input.state {
        return ControlFlow::Poll;
    }
    let mut control_flow = ControlFlow::Poll;
    if let Some(keycode) = input.virtual_keycode {
        match keycode {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => return ControlFlow::Exit,
            VirtualKeyCode::Subtract => renderer.diminish_precision(),
            VirtualKeyCode::Add => renderer.augment_precision(),
            VirtualKeyCode::W => renderer.augment_zoom(),
            VirtualKeyCode::S => renderer.diminish_zoom(),
            VirtualKeyCode::Left => renderer.move_left(),
            VirtualKeyCode::Right => renderer.move_right(),
            VirtualKeyCode::Down => renderer.move_down(),
            VirtualKeyCode::Up => renderer.move_up(),
            VirtualKeyCode::C => renderer.next_color(),
            VirtualKeyCode::T => renderer.switch_automation(),
            VirtualKeyCode::X => renderer.next_fractal_type(),
            _ => control_flow = ControlFlow::Poll,
        }
    }

    control_flow
}
