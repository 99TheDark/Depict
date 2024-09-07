struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tex_idx: u32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) tex_idx: u32,
}

@group(1) @binding(0) var<uniform> scale: vec2<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.uv = in.uv;
    out.pos = vec4<f32>(in.pos * scale, 0.0, 1.0);
    out.tex_idx = in.tex_idx;

    return out;
}

/*@group(0) @binding(0)
var t_tile: texture_2d<f32>;
@group(0) @binding(2)
var t_ui: texture_2d<f32>;

@group(0) @binding(1)
var s_tile: sampler;
@group(0) @binding(3)
var s_ui: sampler;*/

// TODO: Automatically fill these in on the spot or use one large texture atlas
@group(0) @binding(0) var texture1: texture_2d<f32>;
@group(0) @binding(1) var sampler1: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Also needs work
    switch in.tex_idx {
        case 1u {
            return textureSample(texture1, sampler1, in.uv);
        }
        default {
            return in.color;
        }
    }
}
 