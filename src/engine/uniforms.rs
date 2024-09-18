use core::fmt::Debug;

use bytemuck::{cast_slice, Pod, Zeroable};
use glam::{Affine2, Mat3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType,
    BufferUsages, Device, ShaderStages,
};

pub(crate) struct Uniforms {
    pub bind_group: BindGroup,
    pub transformation: Uniform<TransformationData>,
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
pub(crate) struct TransformationData {
    pub transformation: [[f32; 4]; 4],
}

impl TransformationData {
    pub fn update(&mut self, screen: Affine2) {
        let affine = Mat3::from(screen).to_cols_array_2d();

        self.transformation = [
            [affine[0][0], affine[1][0], 0.0, affine[2][0]],
            [affine[0][1], affine[1][1], 0.0, affine[2][1]],
            [0.0, 0.0, 1.0, 0.0],
            [affine[0][2], affine[1][2], 0.0, affine[2][2]],
        ];
    }
}
