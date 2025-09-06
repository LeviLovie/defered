use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Color, ColorTargetState,
    ColorWrites, CommandEncoder, Device, FragmentState, LoadOp, MultisampleState, Operations,
    PipelineLayoutDescriptor, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor, ShaderStages,
    StoreOp, TextureFormat, TextureSampleType, TextureView, TextureViewDimension, VertexState,
    include_wgsl,
};

use crate::renderer::gbuffer::GBuffer;

pub struct Composite {
    pipeline: RenderPipeline,
    bind_group: BindGroup,
    _layers: u32,
}

impl Composite {
    pub fn new(device: &Device, format: TextureFormat, gbuffer: &GBuffer) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../../shaders/composite.wgsl"));
        let sampler = device.create_sampler(&SamplerDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Composite Grid BGL"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2Array,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
            ],
        });

        // Bind group: sampler + the array texture
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Composite Grid BindGroup"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&gbuffer.color_view),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Composite Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Composite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            _layers: gbuffer.layers,
        }
    }

    pub fn execute(&self, encoder: &mut CommandEncoder, view: &TextureView) {
        let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Composite Grid Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..6, 0..1);
    }
}
