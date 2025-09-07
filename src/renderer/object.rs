use getset::{Getters, MutGetters, Setters};
use nalgebra::{Vector2, Vector4};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ObjectRaw {
    pub pos: [f32; 2],  // 8 bytes
    pub size: [f32; 2], // 8 bytes
    pub tint: [u8; 4],  // 4 bytes
    pub rot: f32,       // 4 bytes
    pub bid: u32,       // 4 bytes
    pub tid: u32,       // 4 bytes
                        // 32 bytes total, no padding needed
}

#[derive(Getters, Setters, MutGetters)]
#[get = "pub"]
#[set = "pub"]
#[get_mut = "pub"]
pub struct Object {
    pos: Vector2<f32>,
    size: Vector2<f32>,
    rot: f32,
    tint: Vector4<u8>,
    bid: usize,
    tid: usize,
}

impl Into<ObjectRaw> for &Object {
    fn into(self) -> ObjectRaw {
        ObjectRaw {
            pos: self.pos.into(),
            size: self.size.into(),
            rot: self.rot,
            tint: self.tint.into(),
            bid: self.bid as u32,
            tid: self.tid as u32,
        }
    }
}

impl Object {
    pub fn new() -> Self {
        Self {
            pos: Vector2::new(0.0, 0.0),
            size: Vector2::new(1.0, 1.0),
            rot: 0.0,
            tint: Vector4::new(255, 255, 255, 255),
            bid: 0,
            tid: 0,
        }
    }
}
