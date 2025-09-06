use wgpu::{
    Device, Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};

pub struct GBuffer {
    pub _texture: Texture,
    pub view: TextureView,
    pub depth: TextureView,
    pub format: TextureFormat,
    pub depth_format: TextureFormat,
    pub _size: (u32, u32),
}

impl GBuffer {
    pub fn new(device: &Device, width: u32, height: u32) -> Self {
        let format = TextureFormat::Rgba8Unorm;
        let depth_format = TextureFormat::Depth24Plus;

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("GBuffer Color"),
            size: Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let depth_tex = device.create_texture(&TextureDescriptor {
            label: Some("GBuffer Depth"),
            size: Extent3d {
                width: width,
                height: height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: depth_format,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth = depth_tex.create_view(&Default::default());

        Self {
            _texture: texture,
            view,
            depth,
            format,
            depth_format,
            _size: (width, height),
        }
    }
}
