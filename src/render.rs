use std::iter::once;

use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendState,
    BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, Extent3d, Features,
    FragmentState, FrontFace, ImageCopyTexture, ImageDataLayout, Instance, InstanceDescriptor,
    Limits, LoadOp, MemoryHints, MultisampleState, Operations, Origin3d,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PowerPreference,
    PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceError, TextureAspect,
    TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
    TextureViewDescriptor, TextureViewDimension, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{error::MageError, FontData};

pub(crate) struct RenderState<'a> {
    /// The surface that we'll render to.
    surface: Surface<'a>,

    /// Various configuration options for the surface.
    surface_config: SurfaceConfiguration,

    /// The GPU device that will create and manage our resources.
    device: Device,

    /// The queue that we'll submit render commands to.
    queue: Queue,

    /// The render pipeline for drawing the game.
    render_pipeline: RenderPipeline,

    /// The window that we'll draw to.
    // Added lifetime and made it a reference because God (the compiler) said so
    pub(crate) window: &'a Window,

    /// The texture that contains the foreground color data.
    fg_texture: Texture,

    /// The texture that contains the background color data.
    bg_texture: Texture,

    /// The texture that contains the character data.
    chars_texture: Texture,

    /// The texture that contains the font data.
    font_texture: Texture,

    /// The bind group layout for the textures.
    texture_bind_group_layout: BindGroupLayout,

    // The bind group for the textures.
    texture_bind_group: BindGroup,

    /// The bind group for the uniform data.
    uniform_bind_group: BindGroup,

    /// The size of each character in the font texture.
    font_char_size: (u32, u32),

    /// The size of the surface in characters.
    surface_char_size: (u32, u32),
}

impl<'a> RenderState<'a> {
    pub(crate) async fn new(window: &'a Window, font: FontData) -> Result<Self, MageError> {
        let window_size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        // Unsafe function no longer needed

        let surface = instance.create_surface(window).ok();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: surface.as_ref(),
            })
            .await
            .ok_or(MageError::BadAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Main device"),
                    // Breaking change means that you have to say "required" limits and features
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::Performance,
                },
                None,
            )
            .await?;

        let surface_expected = surface.expect("No surface Found");
        let surface_format = surface_expected
            .get_capabilities(&adapter)
            .formats
            .iter()
            .copied()
            .find(|format| !format.is_srgb())
            .expect("Could not find the surface format");
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::AutoNoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface_expected.configure(&device, &surface_config);

        let font_size = (16 * font.char_width, 16 * font.char_height);
        let surface_size = (
            window_size.width / font.char_width,
            window_size.height / font.char_height,
        );
        let fg_texture = Texture::new(&device, surface_size);
        let bg_texture = Texture::new(&device, surface_size);
        let chars_texture = Texture::new(&device, surface_size);
        let mut font_texture = Texture::new(&device, font_size);

        font_texture.storage.copy_from_slice(font.data.as_slice());
        font_texture.update(&queue);

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
        let texture_bind_group = create_texture_bind_group(
            &device,
            &texture_bind_group_layout,
            &fg_texture,
            &bg_texture,
            &chars_texture,
            &font_texture,
        );

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
        let surface_char_size = (
            window_size.width / font.char_width,
            window_size.height / font.char_height,
        );

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: PipelineCompilationOptions {
                    ..Default::default()
                },
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions {
                    ..Default::default()
                },
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Ok(Self {
            surface: surface_expected,
            surface_config,
            device,
            queue,
            render_pipeline,
            window,
            fg_texture,
            bg_texture,
            chars_texture,
            font_texture,
            texture_bind_group_layout,
            texture_bind_group,
            uniform_bind_group,
            font_char_size,
            surface_char_size,
        })
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);

            let chars_size = (
                new_size.width / self.font_char_size.0,
                new_size.height / self.font_char_size.1,
            );

            if chars_size != self.surface_char_size {
                self.surface_char_size = chars_size;
                self.fg_texture = Texture::new(&self.device, chars_size);
                self.bg_texture = Texture::new(&self.device, chars_size);
                self.chars_texture = Texture::new(&self.device, chars_size);

                self.texture_bind_group = create_texture_bind_group(
                    &self.device,
                    &self.texture_bind_group_layout,
                    &self.fg_texture,
                    &self.bg_texture,
                    &self.chars_texture,
                    &self.font_texture,
                );
            }
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), SurfaceError> {
        self.fg_texture.update(&self.queue);
        self.bg_texture.update(&self.queue);
        self.chars_texture.update(&self.queue);

        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.draw(0..4, 0..1);
        }

        self.queue.submit(once(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub(crate) fn size_in_chars(&self) -> (u32, u32) {
        self.surface_char_size
    }

    pub(crate) fn images(&mut self) -> (&mut [u32], &mut [u32], &mut [u32]) {
        (
            &mut self.fg_texture.storage,
            &mut self.bg_texture.storage,
            &mut self.chars_texture.storage,
        )
    }
}

fn create_texture_bind_group(
    device: &Device,
    texture_bind_group_layout: &BindGroupLayout,
    fg_texture: &Texture,
    bg_texture: &Texture,
    chars_texture: &Texture,
    font_texture: &Texture,
) -> BindGroup {
    let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout: texture_bind_group_layout,
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
    texture_bind_group
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
