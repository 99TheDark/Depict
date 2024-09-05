struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) tex_idx: u32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) tex_idx: u32,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.uv = in.uv;
    out.pos = vec4<f32>(in.pos, 0.0, 1.0);
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    /*switch in.tex_idx {
        case 0u {
            return textureSample(t_tile, s_tile, in.uv);
        }
        case 1u {
            return textureSample(t_ui, s_ui, in.uv);
        }
        default {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
    }*/
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
 