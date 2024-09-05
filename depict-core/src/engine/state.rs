use std::{cell::RefCell, iter, rc::Rc, sync::Arc};

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
        texture::TextureSource,
    },
    input::{keyboard::Keyboard, mouse::Mouse, tracker::Tracker},
};

use super::{
    properties::{Properties, Size},
    renderer::Renderer,
    shader::Shader,
};

pub(crate) struct State<'a> {
    pub(crate) textures: Vec<TextureSource>,
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
    bind_group: BindGroup,
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
        let aspect = size.width as f32 / size.height as f32;

        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        };
        let instance = Instance::new(instance_descriptor);

        let target = unsafe { SurfaceTargetUnsafe::from_window(&window) }.unwrap();
        let surface = unsafe { instance.create_surface_unsafe(target) }.unwrap();

        let adapter_descriptor = RequestAdapterOptionsBase {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = DeviceDescriptor {
            required_features: Features::empty(),
            required_limits: Limits::default(),
            label: Some("Device"),
        };
        let (device, queue) = adapter
            .request_device(&device_descriptor, None)
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
            texture_count: 0,
            textures: Vec::new(),
            device: &device,
            queue: &queue,
            size,
        };

        system.borrow_mut().init(&mut ctx);

        let mut bind_layout_entries = Vec::with_capacity(ctx.texture_count * 2);
        let mut bind_entries = Vec::with_capacity(ctx.texture_count * 2);

        for (idx, texture) in ctx.textures.iter().enumerate() {
            let i = idx * 2;

            bind_layout_entries[i] = BindGroupLayoutEntry {
                binding: i as u32,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    view_dimension: TextureViewDimension::D2,
                    sample_type: TextureSampleType::Float { filterable: true },
                },
                count: None,
            };
            bind_layout_entries[i + 1] = BindGroupLayoutEntry {
                binding: i as u32 + 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            };

            bind_entries[i] = BindGroupEntry {
                binding: i as u32,
                resource: BindingResource::TextureView(&texture.view),
            };

            BindGroupEntry {
                binding: i as u32 + 1,
                resource: BindingResource::Sampler(&texture.sampler),
            };
        }

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &bind_layout_entries[..],
                label: Some("Texture Bind Group Layout"),
            });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &bind_entries[..],
            label: Some("Bind Group"),
        });

        let shader = Shader::new("shaders/shader.wgsl", config.format);
        let pipeline = shader.build(&device, &texture_bind_group_layout);

        let properties = Properties { size, aspect };

        let mouse = Tracker::new(Mouse::new());
        let keyboard = Tracker::new(Keyboard::new());

        let mut renderer = Renderer::new();
        let (count, vertex_buffer, index_buffer) = renderer.build(&device);

        State {
            textures: ctx.textures,
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
            bind_group,
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
        self.system.borrow_mut().render(&mut Context {
            step: ContextStep::Render,
            mouse: &self.mouse,
            keyboard: &self.keyboard,
            renderer: Some(&mut renderer),
            properties: &self.properties,
            window: self.window.clone(),
        });

        renderer.build(&self.device)
    }

    pub fn update(&mut self) {
        self.system.borrow_mut().update(&mut Context {
            step: ContextStep::Update,
            mouse: &self.mouse,
            keyboard: &self.keyboard,
            renderer: None,
            properties: &self.properties,
            window: self.window.clone(),
        });
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        (self.count, self.vertex_buffer, self.index_buffer) = self.build();

        let drawable = self.surface.get_current_texture()?;

        let image_view_descriptor = TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let encoder_descriptor = CommandEncoderDescriptor {
            label: Some("Command Encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&encoder_descriptor);

        let color_attatchment = RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(self.clear_color),
                store: StoreOp::Store,
            },
        };

        let render_pass_descriptor = RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attatchment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
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
