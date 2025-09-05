#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Object {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub layer: u32,
    pub obj_type: u32, // store as u32
    pub special_data: [u32; 4],
}
