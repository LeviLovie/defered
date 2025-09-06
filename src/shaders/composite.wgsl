@group(0) @binding(0) var gbuffer: texture_2d<f32>;
@group(0) @binding(1) var sampler0: sampler;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),

        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );
    return vec4<f32>(pos[i], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(gbuffer, 0));
    let uv = pos.xy / dims;
    let uv_corrected = vec2<f32>(uv.x, 1.0 - uv.y); // Flip Y if needed
    return textureSample(gbuffer, sampler0, uv_corrected);
}
