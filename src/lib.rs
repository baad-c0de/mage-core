mod app;
mod error;
mod render;

use std::time::Duration;

pub use app::*;
pub use error::*;
pub use render::*;

use error::MageError;
use render::RenderState;
use tracing::{error, info};
use wgpu::SurfaceError;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run<A>(mut app: A) -> Result<(), MageError>
where
    A: App + 'static,
{
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(640, 480))
        .with_title("Mage Game")
        .build(&event_loop)?;

    let mut state = RenderState::new(window).await?;

    let mut key_state = KeyState {
        shift: false,
        ctrl: false,
        alt: false,
        vkey: None,
        code: None,
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { window_id, event } if window_id == state.window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_size) => {
                        info!("Resized to {:?}", new_size);
                        state.resize(new_size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size: new_size,
                        ..
                    } => {
                        info!("Resized to {:?}", new_size);
                        state.resize(*new_size);
                    }
                    _ => (),
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window.id() => {
                if present(&mut app, &mut state) == PresentResult::Changed {
                    match state.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => {
                            info!("Surface lost, recreating");
                            state.resize(state.window.inner_size());
                        }
                        Err(SurfaceError::OutOfMemory) => {
                            error!("Out of memory, exiting");
                            *control_flow = ControlFlow::Exit;
                        }
                        Err(e) => error!("Error: {:?}", e),
                    }
                }
            }
            Event::MainEventsCleared => {
                if tick(&mut app, &mut state, &key_state) == TickResult::Quit {
                    *control_flow = ControlFlow::Exit;
                }
                key_state.vkey = None;
                state.window.request_redraw();
            }
            _ => (),
        }
    });
}

fn tick<A>(app: &mut A, state: &mut RenderState, key_state: &KeyState) -> TickResult
where
    A: App,
{
    let (width, height) = state.chars_size();
    let tick_input = TickInput {
        dt: Duration::ZERO,
        width,
        height,
        key: key_state.clone(),
        mouse: None,
    };
    app.tick(tick_input)
}

fn present<A>(app: &mut A, state: &mut RenderState) -> PresentResult
where
    A: App,
{
    let (width, height) = state.chars_size();
    let (fore_image, back_image, text_image) = state.images();

    let present_input = PresentInput {
        width,
        height,
        fore_image,
        back_image,
        text_image,
    };

    app.present(present_input)
}
