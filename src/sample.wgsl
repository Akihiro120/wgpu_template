struct VertexInput {
    @location(0) pos: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> time: f32;

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4f {
    return vec4<f32>(in.pos, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    let r = sin(time) * 0.5 + 0.5;
    let g = sin(time + 2.0) * 0.5 + 0.5;
    let b = sin(time + 4.0) * 0.5 + 0.5;

    return vec4<f32>(r, g, b, 1.0);
}
