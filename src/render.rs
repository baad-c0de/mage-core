use std::iter::once;

use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BufferBindingType, BufferUsages, Color, CommandEncoderDescriptor, Device, DeviceDescriptor,
    Extent3d, Features, ImageCopyTexture, ImageDataLayout, Instance, InstanceDescriptor, Limits,
    LoadOp, Operations, Origin3d, PowerPreference, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterOptions, ShaderStages, Surface, SurfaceConfiguration,
    SurfaceError, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
    TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{error::MageError, FontData};

pub(crate) struct RenderState {
    surface: Surface,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    pub(crate) window: Window,

    fg_texture: Texture,
    bg_texture: Texture,
    chars_texture: Texture,
    font_texture: Texture,
    texture_bind_group_layout: BindGroupLayout,
    texture_bind_group: BindGroup,
    uniform_bind_group: BindGroup,
    font_char_size: (u32, u32),
}

impl RenderState {
    pub(crate) async fn new(window: Window, font: FontData) -> Result<Self, MageError> {
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

        let font_size = (16 * font.char_width, 16 * font.char_height);
        let fg_texture = Texture::new(&device, font_size);
        let bg_texture = Texture::new(&device, font_size);
        let chars_texture = Texture::new(&device, font_size);
        let font_texture = Texture::new(&device, (16 * font.char_width, 16 * font.char_height));

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 3,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: false },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });
        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(
                        &fg_texture
                            .texture
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(
                        &bg_texture
                            .texture
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(
                        &chars_texture
                            .texture
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::TextureView(
                        &font_texture
                            .texture
                            .create_view(&TextureViewDescriptor::default()),
                    ),
                },
            ],
        });

        let uniforms = RenderUniforms {
            font_width: font.char_width,
            font_height: font.char_height,
            _padding: [0; 2],
        };
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer for Render"),
            contents: cast_slice(&[uniforms]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Uniforms bind group layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniforms bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let font_char_size = (font.char_width, font.char_height);

        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            window,
            fg_texture,
            bg_texture,
            chars_texture,
            font_texture,
            texture_bind_group_layout,
            texture_bind_group,
            uniform_bind_group,
            font_char_size,
        })
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), SurfaceError> {
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

    pub(crate) fn chars_size(&self) -> (u32, u32) {
        (10, 16)
    }

    pub(crate) fn images(&self) -> (&Vec<u32>, &Vec<u32>, &Vec<u32>) {
        (
            &self.fg_texture.storage,
            &self.bg_texture.storage,
            &self.chars_texture.storage,
        )
    }
}

struct Texture {
    /// Size of the texture in pixels.
    pub(crate) size: (u32, u32),

    /// The texture itself.
    pub(crate) storage: Vec<u32>,

    /// The WGPU texture object.
    texture: wgpu::Texture,
}

impl Texture {
    fn new(device: &Device, size: (u32, u32)) -> Self {
        let vec_size = (size.0 * size.1) as usize;
        let storage = vec![0; vec_size];

        let texture_size = Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        Self {
            size,
            storage,
            texture,
        }
    }

    fn update(&mut self, queue: &Queue) {
        let (width, height) = self.size;
        queue.write_texture(
            ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            cast_slice(&self.storage),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct RenderUniforms {
    /// The width of a single character in pixels.
    font_width: u32,

    /// The height of a single character in pixels.
    font_height: u32,

    /// Some padding.
    _padding: [u32; 2],
}
