mod composite;
mod geometry;

pub use composite::{Composite, CompositeMode};
pub use geometry::Geometry;

use wgpu::{CommandEncoder, Device, Queue};

use super::{ecs::TextureCache, gbuffer::GBuffer};

pub struct RenderPassData<'a> {
    pub texture_cache: &'a mut TextureCache,
    pub gbuffer: &'a GBuffer,
    pub encoder: &'a mut CommandEncoder,
    pub device: &'a Device,
    pub queue: &'a Queue,
}
