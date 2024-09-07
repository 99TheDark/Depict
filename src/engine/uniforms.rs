use core::fmt::Debug;

use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType,
    BufferUsages, Device, ShaderStages,
};

pub(crate) struct Uniforms {
    pub bind_group: BindGroup,
    pub scale: Uniform<ScaleData>,
}

pub(crate) struct Uniform<T: Copy + Clone + Debug + Pod + Zeroable + PartialEq> {
    pub data: T,
    pub buffer: Buffer,
}

impl<T: Copy + Clone + Debug + Pod + Zeroable + PartialEq> Uniform<T> {
    pub fn new(device: &Device, data: T, label: Option<&str>) -> Self {
        let binding = [data];

        let description = BufferInitDescriptor {
            label,
            contents: cast_slice(&binding),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        };
        let buffer = device.create_buffer_init(&description);

        Self { data, buffer }
    }

    pub fn register<'a>(
        &'a self,
        layout_entries: &mut Vec<BindGroupLayoutEntry>,
        entries: &mut Vec<BindGroupEntry<'a>>,
    ) {
        layout_entries.push(BindGroupLayoutEntry {
            binding: layout_entries.len() as u32,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        entries.push(BindGroupEntry {
            binding: entries.len() as u32,
            resource: self.buffer.as_entire_binding(),
        });
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, PartialEq)]
pub(crate) struct ScaleData {
    pub scale: [f32; 2],
}

impl ScaleData {
    pub fn update(&mut self, width: u32, height: u32) {
        self.scale[0] = 1.0 / width as f32;
        self.scale[1] = -1.0 / height as f32;
    }
}
