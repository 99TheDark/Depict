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

const u32_max = 4294967295u;

@group(1) @binding(0) var<uniform> scale: vec2<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.uv = in.uv;
    out.pos = vec4<f32>(in.pos * scale * 2.0 + vec2<f32>(-1.0, 1.0), 0.0, 1.0);
    out.tex_idx = in.tex_idx;

    return out;
}

// TODO: Automatically fill these in on the spot or use one large texture atlas
@group(0) @binding(0) var texture1: texture_2d<f32>;
@group(0) @binding(1) var sampler1: sampler;

@group(1) @binding(0) var texture_atlas: texture_2d<f32>;
@group(1) @binding(1) var sampler_atlas: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Also needs work
    /*switch in.tex_idx {
        case 0u {
            return textureSample(texture1, sampler1, in.uv);
        }
        default {
            return vec4<f32>(0.0, 0.0, 1.0, 1.0);
            // return in.color;
        }
    }*/
    // Could probably temporarily remove tex_idx, eventually replace with a u8 or something
    if in.tex_idx == 0u {
        return textureSample(texture1, sampler1, in.uv);
    } else {
        return vec4<f32>(0.0, 0.0, 1.0, 1.0);
    }
}
 