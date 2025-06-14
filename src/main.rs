use glow::HasContext;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::{Display, DisplayApiPreference, GetGlDisplay};
use glutin::prelude::*;
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct Game {
    player_position: (f32, f32),
}

impl Game {
    fn new() -> Self {
        Self {
            player_position: (0.0, 0.0),
        }
    }

    fn update(&mut self) {
        // logic here
    }

    fn render(&self, gl: &glow::Context) {
        unsafe {
            gl.clear_color(0.1, 0.1, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();

    let window_builder = WindowBuilder::new()
        .with_title("Rust Game")
        .with_inner_size(LogicalSize::new(800.0, 600.0));

    let template = glutin::config::ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_depth_size(24)
        .with_stencil_size(8)
        .with_transparency(false)
        .compatible_with_native_window();

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| configs.next().unwrap())
        .unwrap();

    let window = window.unwrap();
    let raw_window_handle = window.raw_window_handle();

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None)) // GLES2 requested here
        .build(Some(raw_window_handle));

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some((2, 0))))
        .build(Some(raw_window_handle));

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .or_else(|_| gl_display.create_context(&gl_config, &fallback_context_attributes))
            .unwrap()
    };

    let attrs = window.build_surface_attributes(<_>::default());
    let surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current_gl_context.make_current(&surface).unwrap();

    let glow_context = unsafe {
        glow::Context::from_loader_function(|s| gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap()) as *const _)
    };

    let mut game = Game::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state,
                            ..
                        },
                    ..
                } => {
                    use winit::event::ElementState::*;
                    match (state, key) {
                        (Pressed, VirtualKeyCode::W) => game.player_position.1 -= 0.1,
                        (Pressed, VirtualKeyCode::S) => game.player_position.1 += 0.1,
                        (Released, VirtualKeyCode::W) => game.player_position.1 += 0.1,
                        (Released, VirtualKeyCode::S) => game.player_position.1 -= 0.1,
                        _ => {}
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                game.update();
                game.render(&glow_context);
                surface.swap_buffers(&gl_context).unwrap();
            }
            _ => {}
        }
    });
}
