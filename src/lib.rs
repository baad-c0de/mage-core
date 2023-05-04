use std::iter::once;

use thiserror::Error;
use tracing::{error, info};
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, CreateSurfaceError, Device, DeviceDescriptor,
    Features, Instance, InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, RequestDeviceError,
    Surface, SurfaceConfiguration, SurfaceError, TextureUsages, TextureViewDescriptor,
};
use winit::{
    dpi::PhysicalSize,
    error::OsError,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(Debug, Error)]
pub enum MageError {
    #[error("unable to open window")]
    WindowError(#[from] OsError),

    #[error("unable to create rendering surface")]
    CreateSurfaceError(#[from] CreateSurfaceError),

    #[error("unable to create GPU adapter")]
    BadAdapter,

    #[error("unable to create GPU device")]
    BadDevice(#[from] RequestDeviceError),
}

struct RenderState {
    pub surface: Surface,
    pub surface_config: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    pub window: Window,
}

pub async fn run() -> Result<(), MageError> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(640, 480))
        .with_title("Mage Game")
        .build(&event_loop)?;

    let mut state = RenderState::new(window).await?;

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
            _ => (),
        }
    });
}

impl RenderState {
    async fn new(window: Window) -> Result<Self, MageError> {
        let window_size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(MageError::BadAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Main device"),
                    features: Features::empty(),
                    limits: Limits::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|format| !format.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            window,
        })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
