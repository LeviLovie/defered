struct Object {
    pos: vec2<f32>,
    size: vec2<f32>,
    color: vec4<f32>,
    layer: u32,
};

@group(0) @binding(0)
var<storage, read> objects: array<Object>;

var<private> quad_positions: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0)
);

struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32,
           @builtin(instance_index) ii: u32) -> VSOut {
    let quad = quad_positions[vi];
    let obj = objects[ii];
    let aspect_ratio = 800.0 / 600.0;

    var out: VSOut;
    out.pos = vec4<f32>(
        (obj.pos.x / aspect_ratio) + (quad.x * obj.size.x / aspect_ratio),
        obj.pos.y + (quad.y * obj.size.y),
        0.0, 1.0
    );
    out.color = obj.color;
    return out;
}

@fragment
fn fs_main(input: VSOut) -> @location(0) vec4<f32> {
    return input.color;
}
