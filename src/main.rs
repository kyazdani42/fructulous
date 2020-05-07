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
    renderer.draw();

    el.run(move |event, _, control_flow| {
        *control_flow = match event {
            Event::WindowEvent { event, .. } => window_events(event, &mut renderer),
            Event::RedrawRequested(_) => {
                renderer.draw();
                ControlFlow::Wait
            }
            _ => ControlFlow::Wait,
        };
    });
}

fn window_events(event: WindowEvent, renderer: &mut Renderer) -> ControlFlow {
    match event {
        // WindowEvent::Resized(size) => resize(size, renderer),
        WindowEvent::KeyboardInput { input, .. } => handle_keycodes(input, renderer),
        _ => ControlFlow::Wait,
    }
}

// fn resize(size: PhysicalSize<u32>, renderer: &mut Renderer) -> ControlFlow {
//     renderer.context.resize(size);
//     let (width, height) = (size.cast().width, size.cast().height);
//     renderer.camera.update_viewport(width, height);
//     unsafe { gl::Viewport(0, 0, width as i32, height as i32) }
//     ControlFlow::Wait
// }

fn handle_keycodes(input: KeyboardInput, renderer: &mut Renderer) -> ControlFlow {
    if let glutin::event::ElementState::Released = input.state {
        return ControlFlow::Wait;
    }
    let mut control_flow = ControlFlow::Poll;
    if let Some(keycode) = input.virtual_keycode {
        match keycode {
            VirtualKeyCode::Escape | VirtualKeyCode::Q => return ControlFlow::Exit,
            VirtualKeyCode::Subtract => {
                renderer.diminish_precision();
                renderer.draw();
            }
            VirtualKeyCode::Add => {
                renderer.augment_precision();
                renderer.draw();
            }
            VirtualKeyCode::W => {
                renderer.augment_zoom();
                renderer.draw();
            }
            VirtualKeyCode::S => {
                renderer.diminish_zoom();
                renderer.draw();
            }
            VirtualKeyCode::Left => {
                renderer.move_left();
                renderer.draw();
            }
            VirtualKeyCode::Right => {
                renderer.move_right();
                renderer.draw();
            }
            VirtualKeyCode::Down => {
                renderer.move_down();
                renderer.draw();
            }
            VirtualKeyCode::Up => {
                renderer.move_up();
                renderer.draw();
            }
            _ => control_flow = ControlFlow::Wait,
        }
    }

    control_flow
}
