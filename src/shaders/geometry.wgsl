struct Object {
    pos: vec2<f32>,
    size: vec2<f32>,
    color: vec4<f32>,
};

struct Camera {
    pos: vec2<f32>,
    size: vec2<f32>,
}

struct Frame {
    size: vec2<u32>,
}

@group(0) @binding(0) var<storage, read> objects: array<Object>;
@group(1) @binding(0) var<uniform> frame: Frame;
@group(1) @binding(1) var<uniform> camera: Camera;

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

    let pixel_pos = obj.pos + quad * obj.size;

    let cam_x = pixel_pos.x - camera.pos.x;
    let cam_y = pixel_pos.y - camera.pos.y;

    let norm_x = cam_x / camera.size.x;
    let norm_y = cam_y / camera.size.y;

    let ndc_x = norm_x * 2.0 - 1.0;
    let ndc_y = 1.0 - norm_y * 2.0;

    var out: VSOut;
    out.pos = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.color = obj.color;

    return out;
}

@fragment
fn fs_main(input: VSOut) -> @location(0) vec4<f32> {
    return input.color;
}
