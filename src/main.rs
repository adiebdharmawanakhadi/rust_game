use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gles2::{
    glium::{Gl creative, Gl, Surface},
    gles2::types::GLContext,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
       .with_title("Rust Game")
       .with_dimensions((800, 600))
       .build(&event_loop)
       .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event,.. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}