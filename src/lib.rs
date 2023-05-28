pub mod app;
pub mod colour;
pub mod config;
pub mod error;
pub mod image;
pub mod input;
pub mod present;
pub mod render;

use std::cmp::max;

use chrono::{Duration, Local};
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
use winit_fullscreen::WindowFullScreen;

use crate::input::ShiftState;

pub use app::*;
pub use colour::*;
pub use config::*;
pub use error::*;
pub use input::*;
pub use present::*;

pub async fn run<A>(mut app: A, config: Config) -> Result<(), MageError>
where
    A: App + 'static,
{
    //
    // Load font data
    //

    let font_data = match config.font {
        Font::Default => load_font_image(include_bytes!("font1.png"))?,
        Font::Custom(font) => font,
    };

    // Adjust the dimensions of the window to fit character cells exactly.
    let width = max(
        MIN_WINDOW_SIZE.0 * font_data.char_width,
        config.inner_size.0,
    ) / font_data.char_width
        * font_data.char_width;
    let height = max(
        MIN_WINDOW_SIZE.1 * font_data.char_height,
        config.inner_size.1,
    ) / font_data.char_height
        * font_data.char_height;

    info!(
        "Window size (in characters): {}x{}",
        width / font_data.char_width,
        height / font_data.char_height
    );

    //
    // Set up window, game state and event loop
    //

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(width, height))
        .with_title(config.title.unwrap_or("Mage Game".to_string()))
        .with_min_inner_size(PhysicalSize::new(
            MIN_WINDOW_SIZE.0 * font_data.char_width,
            MIN_WINDOW_SIZE.1 * font_data.char_height,
        ))
        .build(&event_loop)?;

    let mut render_state = RenderState::new(window, font_data).await?;
    let mut shift_state = ShiftState::new();

    let mut current_time = Local::now();

    //
    // Run the game loop
    //

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { window_id, event } if window_id == render_state.window.id() => {
                match event {
                    // Detect window close and escape key for application exit
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

                    // Detect ALT+ENTER for fullscreen toggle
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Return),
                                ..
                            },
                        ..
                    } if shift_state.alt_only() => {
                        render_state.window.toggle_fullscreen();
                    }

                    // Detect window resize and scale factor change.  When this happens, the
                    // GPU surface is lost and must be recreated.
                    WindowEvent::Resized(new_size) => {
                        info!("Resized to {:?}", new_size);
                        render_state.resize(new_size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size: new_size,
                        ..
                    } => {
                        info!("Resized to {:?}", new_size);
                        render_state.resize(*new_size);
                    }

                    // Detect shift keys for shift state
                    WindowEvent::ModifiersChanged(modifiers) => {
                        shift_state.update(modifiers);
                    }

                    _ => (),
                }
            }
            Event::RedrawRequested(window_id) if window_id == render_state.window.id() => {
                if present(&mut app, &mut render_state) == PresentResult::Changed {
                    match render_state.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost) => {
                            info!("Surface lost, recreating");
                            render_state.resize(render_state.window.inner_size());
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
                let new_time = Local::now();
                let dt = new_time - current_time;
                current_time = new_time;

                if tick(&mut app, &mut render_state, dt) == TickResult::Quit {
                    *control_flow = ControlFlow::Exit;
                }
                render_state.window.request_redraw();
            }
            _ => (),
        }
    });
}

fn tick<A>(app: &mut A, state: &mut RenderState, dt: Duration) -> TickResult
where
    A: App,
{
    let (width, height) = state.size_in_chars();
    let tick_input = TickInput { dt, width, height };
    app.tick(tick_input)
}

fn present<A>(app: &mut A, state: &mut RenderState) -> PresentResult
where
    A: App,
{
    let (width, height) = state.size_in_chars();
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
