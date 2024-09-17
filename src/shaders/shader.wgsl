struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) atlas_idx: u32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) atlas_idx: u32,
}

const u32_max = 4294967295u;

@group(0) @binding(0) var<uniform> scale: vec2<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.uv = in.uv;
    out.pos = vec4<f32>(in.pos * scale * 2.0 + vec2<f32>(-1.0, 1.0), 0.0, 1.0);
    out.atlas_idx = in.atlas_idx;

    return out;
}

@group(1) @binding(0) var image_atlas: texture_2d<f32>;
@group(1) @binding(1) var image_sampler: sampler;

@group(2) @binding(0) var font_atlas: texture_2d<f32>;
@group(2) @binding(1) var font_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    switch in.atlas_idx {
        case 0u: {
            return textureSample(image_atlas, image_sampler, in.uv);
        }
        case 1u: {
            return textureSample(font_atlas, font_sampler, in.uv);
        }
        default: {
            return in.color;
        }
    }
}
 