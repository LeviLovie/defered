@group(0) @binding(0) var sampler0: sampler;
@group(0) @binding(1) var textures: texture_2d_array<f32>;

struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VSOut {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );
    var uv_coords = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0)
    );

    var out: VSOut;
    out.pos = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uv_coords[vertex_index];
    return out;
}

@fragment
fn fs_main(in: VSOut) -> @location(0) vec4<f32> {
    let layer_count = textureNumLayers(textures);

    let grid_cols = u32(ceil(sqrt(f32(layer_count))));
    let grid_rows = u32(ceil(f32(layer_count) / f32(grid_cols)));

    let col = u32(in.uv.x * f32(grid_cols));
    let row = u32(in.uv.y * f32(grid_rows));

    let clamped_col = min(col, grid_cols - 1u);
    let clamped_row = min(row, grid_rows - 1u);

    let flipped_row = grid_rows - 1u - clamped_row;
    let layer = flipped_row * grid_cols + clamped_col;
    if (layer >= layer_count) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let local_uv = vec2<f32>(
        fract(in.uv.x * f32(grid_cols)),
        fract(in.uv.y * f32(grid_rows))
    );

    let color = textureSample(textures, sampler0, local_uv, layer);

    return color;
}
