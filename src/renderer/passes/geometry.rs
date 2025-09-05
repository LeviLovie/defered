use wgpu::{util::DeviceExt, PipelineCompilationOptions};

use crate::renderer::{gbuffer::GBuffer, object::Object};

pub struct Geometry {
    pipeline: wgpu::RenderPipeline,
    object_bind_layout: wgpu::BindGroupLayout,
}

impl Geometry {
    pub fn new(device: &wgpu::Device, gbuffer: &GBuffer) -> Self {
        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../shaders/geomtery.wgsl"));

        let object_bind_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Object Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Geometry Pipeline Layout"),
            bind_group_layouts: &[&object_bind_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Geometry Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: gbuffer.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: gbuffer.depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            object_bind_layout,
        }
    }

    pub fn execute(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        gbuffer: &GBuffer,
        objects: &[Object],
        device: &wgpu::Device,
    ) {
        let object_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Buffer"),
            contents: bytemuck::cast_slice(objects),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let object_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Object Bind Group"),
            layout: &self.object_bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: object_buffer.as_entire_binding(),
            }],
        });

        for layer in 0..gbuffer.layers {
            let view = &gbuffer.views[layer as usize];
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Geometry Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &gbuffer.depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &object_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }
    }
}
