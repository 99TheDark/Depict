use std::{cell::RefCell, collections::HashMap, iter, rc::Rc, sync::Arc};

use bytemuck::cast_slice;
use wgpu::{
    Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, Color, CommandEncoderDescriptor,
    Device, DeviceDescriptor, Features, IndexFormat, Instance, InstanceDescriptor, Limits, LoadOp,
    Operations, PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RequestAdapterOptionsBase, SamplerBindingType, ShaderStages, StoreOp, Surface,
    SurfaceConfiguration, SurfaceError, SurfaceTargetUnsafe, TextureSampleType, TextureUsages,
    TextureViewDescriptor, TextureViewDimension,
};
use winit::window::Window;

use crate::{
    core::{
        context::{Context, ContextStep, PartialContext},
        settings::Settings,
        system::System,
    },
    graphics::{
        asset::{Assets, FontAsset},
        atlas::Atlas,
    },
    input::{keyboard::Keyboard, mouse::Mouse, tracker::Tracker},
};

use super::{
    properties::{Properties, Size},
    renderer::Renderer,
    shader::Shader,
    uniforms::{ScaleData, Uniform, Uniforms},
};

pub(crate) struct State<'a> {
    pub(crate) size: Size,
    pub(crate) assets: Assets,
    pub(crate) instance: Instance,
    pub(crate) surface: Surface<'a>,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) config: SurfaceConfiguration,
    pub(crate) window: Arc<Window>,
    count: u32,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: RenderPipeline,
    texture_bind_group: BindGroup,
    uniforms: Uniforms,
    pub(crate) properties: Properties,
    pub(crate) mouse: Tracker<Mouse>,
    pub(crate) keyboard: Tracker<Keyboard>,
    pub(crate) clear_color: Color,
    system: Rc<RefCell<dyn System<'a>>>,
}

impl<'a> State<'a> {
    pub async fn new(
        window: Arc<Window>,
        system: Rc<RefCell<dyn System<'a>>>,
        settings: &Settings,
    ) -> State<'a> {
        let size = Size::from_physical(window.inner_size());

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let target = unsafe { SurfaceTargetUnsafe::from_window(&window) }.unwrap();
        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptionsBase {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    label: Some("Device"),
                },
                None,
            )
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let mut ctx = PartialContext {
            img_sources: Vec::new(),
            font_sources: Vec::new(),
            size,
        };

        system.borrow_mut().init(&mut ctx);

        let max_size = Limits::default().max_texture_dimension_2d;
        let image_atlas = Atlas::new(&device, &queue, max_size, max_size, ctx.img_sources);
        let font_atlas = Atlas::new(&device, &queue, max_size, max_size, Vec::new());

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
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
                    resource: BindingResource::TextureView(&image_atlas.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&image_atlas.sampler),
                },
            ],
        });

        let assets = Assets {
            images: image_atlas,
            fonts: FontAsset {
                fonts: HashMap::new(),
                atlas: font_atlas,
            },
        };

        let mut uniform_layout_entries = Vec::new();
        let mut uniform_entries = Vec::new();

        let scale_uniform = Uniform::new(
            &device,
            ScaleData {
                scale: [1.0 / size.width as f32, 1.0 / size.height as f32],
            },
            Some("Scale Uniform"),
        );

        scale_uniform.register(&mut uniform_layout_entries, &mut uniform_entries);

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &uniform_layout_entries[..],
                label: Some("Uniform Bind Group Layout"),
            });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &uniform_entries[..],
            label: Some("Uniform Bind Group"),
        });

        let uniforms = Uniforms {
            bind_group: uniform_bind_group,
            scale: scale_uniform,
        };

        let shader = Shader::new("shaders/shader.wgsl", config.format);
        let pipeline = shader.build(
            &device,
            &[&texture_bind_group_layout, &uniform_bind_group_layout],
        );

        let properties = Properties {
            size,
            aspect: size.aspect(), // Could just be combined...
        };

        let mouse = Tracker::new(Mouse::new());
        let keyboard = Tracker::new(Keyboard::new());

        let mut renderer = Renderer::new();
        let (count, vertex_buffer, index_buffer) = renderer.build(&device);

        State {
            size,
            assets,
            instance,
            surface,
            device,
            queue,
            config,
            window,
            count,
            vertex_buffer,
            index_buffer,
            pipeline,
            texture_bind_group,
            uniforms,
            properties,
            mouse,
            keyboard,
            clear_color: Color {
                r: settings.background.red as f64,
                g: settings.background.green as f64,
                b: settings.background.blue as f64,
                a: 1.0 - settings.background.alpha as f64,
            },
            system,
        }
    }

    fn build(&mut self) -> (u32, Buffer, Buffer) {
        let mut renderer = Renderer::new();
        let mut ctx = Context {
            step: ContextStep::Render,
            size: self.size,
            assets: &self.assets,
            window_size: self.properties.size,
            mouse: &self.mouse,
            keyboard: &self.keyboard,
            renderer: Some(&mut renderer),
            renderables: Vec::new(),
            window: self.window.clone(),
        };

        self.system.borrow_mut().render(&mut ctx);
        ctx.render();

        renderer.build(&self.device)
    }

    pub fn update(&mut self) {
        self.system.borrow_mut().update(&mut Context {
            step: ContextStep::Update,
            size: self.size,
            assets: &self.assets,
            window_size: self.properties.size,
            mouse: &self.mouse,
            keyboard: &self.keyboard,
            renderer: None,
            renderables: Vec::new(),
            window: self.window.clone(),
        });
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        (self.count, self.vertex_buffer, self.index_buffer) = self.build();

        let drawable = self.surface.get_current_texture()?;

        let image_view = drawable
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        let color_attatchment = RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(self.clear_color),
                store: StoreOp::Store,
            },
        };

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attatchment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniforms.bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.count, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        drawable.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: Size) {
        if new_size.width > 0 && new_size.height > 0 {
            self.properties.size = new_size;
            (self.config.width, self.config.height) = (new_size.width, new_size.height);

            self.surface.configure(&self.device, &self.config);

            // This code is gross but idk how to fix it
            self.uniforms
                .scale
                .data
                .update(new_size.width, new_size.height);
            self.queue.write_buffer(
                &self.uniforms.scale.buffer,
                0,
                cast_slice(&[self.uniforms.scale.data]),
            );
        }
    }

    fn update_surface(&mut self) {
        let target = unsafe { SurfaceTargetUnsafe::from_window(&self.window) }.unwrap();
        self.surface = unsafe { self.instance.create_surface_unsafe(target) }.unwrap();
    }

    pub fn reload(&mut self) {
        self.update_surface();
        self.resize(self.properties.size);
    }
}
