struct Object {
    pos: vec2<f32>,
    size: vec2<f32>,
    layer: u32,
    color: vec4<f32>,
    special_data: vec4<u32>,
};

@group(0) @binding(0)
var<storage, read> objects: array<Object>;

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    return vec4<f32>(pos[vid], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    var output: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    for (var i: u32 = 0u; i < arrayLength(&objects); i = i + 1u) {
        let obj = objects[i];

        if (obj.layer != 0u) { continue; } // optional: only render layer 0 for testing

        let min = obj.pos;
        let max = obj.pos + obj.size;

        let frag = frag_coord.xy;

        if (all(frag >= min) && all(frag <= max)) {
            output = obj.color;
        }
    }

    return output;
}
// @fragment
// fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
//     var output: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
// 
//     for (var i: u32 = 0u; i < arrayLength(&objects); i = i + 1u) {
//         let obj = objects[i];
// 
//         let min = obj.pos;
//         let max = obj.pos + obj.size;
// 
//         if (all(frag_coord.xy >= min) && all(frag_coord.xy <= max)) {
//             output = obj.color;
//         }
//     }
// 
//     return output;
// }
