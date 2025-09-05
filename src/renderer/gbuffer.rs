use wgpu::TextureUsages;

pub struct GBuffer {
    pub views: Vec<wgpu::TextureView>,
    pub views_texture_array: wgpu::TextureView,
    pub depth: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
    pub depth_format: wgpu::TextureFormat,
    pub _size: (u32, u32),
    pub layers: u32,
}

impl GBuffer {
    pub fn new(device: &wgpu::Device, width: u32, height: u32, layers: u32) -> Self {
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let depth_format = wgpu::TextureFormat::Depth24Plus;

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("GBuffer Color Array"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: layers,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let mut views = Vec::new();
        for i in 0..layers {
            views.push(texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some(&format!("GBuffer Color View {}", i)),
                format: Some(format),
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: Some(1),
                base_array_layer: i,
                array_layer_count: Some(1),
                ..Default::default()
            }));
        }

        let views_texture_array = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            array_layer_count: Some(layers),
            ..Default::default()
        });

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
                usage: TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default());

        Self {
            views,
            views_texture_array,
            depth,
            format,
            depth_format,
            _size: (width, height),
            layers,
        }
    }
}
