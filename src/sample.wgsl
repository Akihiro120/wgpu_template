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
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
