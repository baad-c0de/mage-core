pub mod app;
pub mod colour;
pub mod config;
pub mod error;
pub mod image;
pub mod input;
pub mod present;
pub mod render;

use chrono::{Duration, Local};
use error::MageError;
use render::RenderState;
use tracing::{error, info};
use wgpu::SurfaceError;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
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

/// Returns the value aligned down to the nearest multiple of the alignment.
fn align_down(value: u32, alignment: u32) -> u32 {
    value / alignment * alignment
}

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

    //
    // Calculate the window size and font scale based on the font data and
    // window size mode.
    //
    let (width, height, scale, snap_size) = match config.window_size {
        WindowSize::FixedCellSize(window_width_pixels, window_height_pixels) => {
            let width = align_down(window_width_pixels, font_data.char_width);
            let height = align_down(window_height_pixels, font_data.char_height);
            (
                width,
                height,
                1u32,
                (font_data.char_width, font_data.char_height),
            )
        }
        WindowSize::FixedCellDimensions(window_width_cells, window_height_cells) => {
            let width = window_width_cells * font_data.char_width;
            let height = window_height_cells * font_data.char_height;
            (width, height, 1u32, (width, height))
        }
        WindowSize::FixedWindowSize(window_width_cells, window_height_cells, cell_scale) => {
            let width = window_width_cells * font_data.char_width * u32::from(cell_scale);
            let height = window_height_cells * font_data.char_height * u32::from(cell_scale);
            (width, height, u32::from(cell_scale), (0u32, 0u32))
        }
    };

    // Adjust the dimensions of the window to fit character cells exactly.
    info!(
        "Window size (in characters): {}x{}; scale: {}; snap size: {:?}",
        width / font_data.char_width,
        height / font_data.char_height,
        scale,
        snap_size
    );
    info!("Window size (in pixels): {width}x{height}");

    //
    // Set up window, game state and event loop
    //

    let event_loop = EventLoop::new()?;

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(width, height))
        .with_title(config.title.unwrap_or("Mage Game".to_string()))
        .with_min_inner_size(PhysicalSize::new(
            MIN_WINDOW_SIZE.0 * font_data.char_width,
            MIN_WINDOW_SIZE.1 * font_data.char_height,
        ))
        .build(&event_loop)?;

    let mut render_state = RenderState::new(window, font_data, scale).await?;
    let mut shift_state = ShiftState::new();

    let mut current_time = Local::now();

    //
    // Run the game loop
    //

    let _ = event_loop.run(move |event, ev_loop| {
        ev_loop.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { window_id, event } if window_id == render_state.window.id() => {
                match event {
                    // Detect window close and escape key for application exit
                    WindowEvent::CloseRequested => ev_loop.exit(),
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,

                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => ev_loop.exit(),

                    // Detect ALT+ENTER for fullscreen toggle
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Enter),
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
                    WindowEvent::ScaleFactorChanged { .. } => {
                        let new_size = render_state.window.inner_size();
                        info!("Resized to {:?}", new_size);
                        render_state.resize(new_size);
                    }

                    // Detect shift keys for shift state
                    WindowEvent::ModifiersChanged(modifiers) => {
                        shift_state.update(modifiers.state());
                    }

                    WindowEvent::RedrawRequested => {
                        if present(&mut app, &mut render_state) == PresentResult::Changed {
                            match render_state.render() {
                                Ok(_) => {}
                                Err(SurfaceError::Lost) => {
                                    info!("Surface lost, recreating");
                                    render_state.resize(render_state.window.inner_size());
                                }
                                Err(SurfaceError::OutOfMemory) => {
                                    error!("Out of memory, exiting");
                                    ev_loop.exit();
                                }
                                Err(e) => error!("Error: {:?}", e),
                            }
                        }
                    }

                    _ => (),
                }
            }
            Event::AboutToWait => {
                let new_time = Local::now();
                let dt = new_time - current_time;
                current_time = new_time;

                if tick(&mut app, &mut render_state, dt) == TickResult::Quit {
                    ev_loop.exit();
                }
                render_state.window.request_redraw();
            }
            _ => (),
        }
    });

    Ok(())
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
