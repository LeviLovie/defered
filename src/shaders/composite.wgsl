struct Mode {
    mode: u32,
    layers: u32,
};

@group(0) @binding(0) var gbuffer_color: texture_2d_array<f32>;
@group(0) @binding(1) var gbuffer_sampler: sampler;
@group(0) @binding(2) var<uniform> mode: Mode;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>( 3.0,  1.0),
        vec2<f32>(-1.0,  1.0)
    );
    return vec4<f32>(pos[i], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(gbuffer_color, 0).xy);
    let uv = pos.xy / dims;

    if (mode.mode == 0u) {
        // Composite all layers front to back
        var out_color: vec4<f32> = vec4<f32>(0.0);
        let layer_count = textureNumLayers(gbuffer_color);
        for (var i: u32 = 0u; i < layer_count; i = i + 1u) {
            let c = textureSample(gbuffer_color, gbuffer_sampler, uv, i);
            out_color = c * c.a + out_color * (1.0 - c.a);
        }
        return out_color;
    } else {
        // Grid example for 16 layers
        let grid_size = u32(sqrt(f32(mode.layers)));
        let cell_uv = fract(uv * vec2<f32>(f32(grid_size), f32(grid_size)));
        let x = u32(uv.x * f32(grid_size));
        let y = u32(uv.y * f32(grid_size));
        let layer = y * grid_size + x;
        return textureSample(gbuffer_color, gbuffer_sampler, cell_uv, layer);
    }
}
