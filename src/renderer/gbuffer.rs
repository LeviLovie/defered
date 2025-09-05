pub struct GBuffer {
    pub _color: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub _format: wgpu::TextureFormat,
    pub depth_format: wgpu::TextureFormat,
    pub _size: (u32, u32),
}

impl GBuffer {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let depth_format = wgpu::TextureFormat::Depth24Plus;

        let color = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("GBuffer Color"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());

        let depth = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("GBuffer Depth"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: depth_format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default());

        Self {
            _color: color,
            depth,
            _format: format,
            depth_format,
            _size: (width, height),
        }
    }
}
