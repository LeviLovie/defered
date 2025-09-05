use wgpu::util::DeviceExt;

use crate::renderer::gbuffer::GBuffer;

#[allow(unused)]
#[derive(Clone)]
pub enum RenderState {
    Combine,
    Grid,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ModeUniform {
    mode: u32,
    layers: u32,
}

impl ModeUniform {
    fn new(mode: RenderState, layers: u32) -> Self {
        let mode_value = match mode {
            RenderState::Combine => 0,
            RenderState::Grid => 1,
        };
        Self {
            mode: mode_value,
            layers,
        }
    }
}

pub struct Composite {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    layers: u32,
    render_state: RenderState,
    mode_buffer: wgpu::Buffer,
}

impl Composite {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        gbuffer: &GBuffer,
        render_state: RenderState,
    ) -> Self {
        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../shaders/composite.wgsl"));

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let mode_uniform = ModeUniform::new(render_state.clone(), gbuffer.layers);
        let mode_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mode Uniform"),
            contents: bytemuck::bytes_of(&mode_uniform),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Composite BGL"),
            entries: &[
                // GBuffer color
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Mode uniform
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Use the texture array view for all layers
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Composite BindGroup"),
            layout: &bind_group_layout,
            entries: &[
                // GBuffer color
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&gbuffer.views_texture_array),
                },
                // Sampler
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                // Mode uniform
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: mode_buffer.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Composite Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Composite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            layers: gbuffer.layers,
            render_state,
            mode_buffer,
        }
    }

    pub fn execute(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        queue: &mut wgpu::Queue,
        view: &wgpu::TextureView,
    ) {
        queue.write_buffer(
            &self.mode_buffer,
            0,
            bytemuck::bytes_of(&ModeUniform::new(self.render_state.clone(), self.layers)),
        );

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Composite Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            ..Default::default()
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);

        rpass.draw(0..3, 0..1);
    }
}
