use learn_wgpu::state::State;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let win = WindowBuilder::new()
        .build(&event_loop)
        .expect("window build fail");

    let mut state = State::new(&win).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == win.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }

            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }

            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(*new_inner_size);
            }

            WindowEvent::CursorMoved { .. } | WindowEvent::KeyboardInput { .. } => {
                state.input(&event);
            }

            _ => {}
        },

        Event::RedrawRequested(window_id) if window_id == win.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            win.request_redraw();
        }

        _ => {}
    })
}
