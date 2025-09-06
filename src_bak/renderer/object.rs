// Object data structure for rendering

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Object {
    pub pos: [f32; 2],      // 8 Bytes
    pub size: [f32; 2],     // 8 Bytes
    pub color: [f32; 4],    // 16 Bytes
    pub layer: u32,         // 4 Bytes = 36 Bytes
    pub _padding: [u32; 3], // 12 Bytes to make it 48 Bytes (multiple of 16)
}
