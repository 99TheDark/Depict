use std::fs;
use std::{env::current_dir, mem::size_of};

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupLayout, BlendState, BufferAddress, ColorTargetState, ColorWrites, Device,
    FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

struct Attributes {
    pub attributes: Vec<VertexAttribute>,
    offset: u64,
}

impl Attributes {
    fn new() -> Self {
        Self {
            attributes: Vec::new(),
            offset: 0,
        }
    }

    fn add(&mut self, format: VertexFormat) {
        let attribute = VertexAttribute {
            offset: self.offset,
            shader_location: self.attributes.len() as u32,
            format,
        };

        self.attributes.push(attribute);
        self.offset += format.size();
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
    pub uv: [f32; 2],
    pub tex_idx: u32,
}

impl Vertex {
    fn description() -> VertexBufferLayout<'static> {
        let mut attributes = Attributes::new();
        attributes.add(VertexFormat::Float32x2); // Position
        attributes.add(VertexFormat::Float32x4); // Color
        attributes.add(VertexFormat::Float32x2); // UV
        attributes.add(VertexFormat::Uint32); // Texture ID

        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: attributes.attributes.leak(),
        }
    }
}

pub struct Shader {
    shader_name: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: TextureFormat,
}

impl Shader {
    pub fn new(shader_name: &str, pixel_format: TextureFormat) -> Shader {
        return Shader {
            shader_name: shader_name.to_string(),
            vertex_entry: "vs_main".to_string(),
            fragment_entry: "fs_main".to_string(),
            pixel_format,
        };
    }

    pub fn build(
        &self,
        device: &Device,
        texture_bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let mut os_path = current_dir().unwrap();
        os_path.push("src/");
        os_path.push(&self.shader_name);

        let file_path = os_path.into_os_string().into_string().unwrap();

        let source_code = fs::read_to_string(&file_path)
            .expect(format!("Cannot read source code in {}", &file_path).as_str());

        let module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(source_code.into()),
        };
        let module = device.create_shader_module(module_descriptor);

        let layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        };
        let layout = device.create_pipeline_layout(&layout_descriptor);

        let render_targets = [Some(ColorTargetState {
            format: self.pixel_format,
            blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        })];

        let pipeline_descriptor = RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: VertexState {
                module: &module,
                entry_point: &self.vertex_entry,
                buffers: &[Vertex::description()],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &module,
                entry_point: &self.fragment_entry,
                targets: &render_targets,
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: /*Some(Face::Back)*/ None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: 1,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };
        device.create_render_pipeline(&pipeline_descriptor)
    }
}
