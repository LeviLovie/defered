use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoder, CompareFunction, DepthStencilState, Device,
    FragmentState, LoadOp, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderStages, StoreOp, VertexState,
};

use crate::renderer::{gbuffer::GBuffer, object::Object};

pub struct Geometry {
    pipeline: RenderPipeline,
    object_bind_layout: BindGroupLayout,
}

impl Geometry {
    pub fn new(device: &Device, gbuffer: &GBuffer) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../../shaders/geometry.wgsl"));

        let object_bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Object Bind Group Layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Geometry Pipeline Layout"),
            bind_group_layouts: &[&object_bind_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Geometry Pipeline"),
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
                    format: gbuffer.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(DepthStencilState {
                format: gbuffer.depth_format,
                depth_write_enabled: false,
                depth_compare: CompareFunction::Less,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
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
        encoder: &mut CommandEncoder,
        gbuffer: &GBuffer,
        objects: &[Object],
        device: &Device,
        layer: u32,
    ) {
        let object_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Object Buffer"),
            contents: bytemuck::cast_slice(objects),
            usage: BufferUsages::STORAGE,
        });

        let object_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Object Bind Group"),
            layout: &self.object_bind_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: object_buffer.as_entire_binding(),
            }],
        });

        let color_view = &gbuffer.color_layer_view(layer);
        let depth_view = &gbuffer.depth_layer_view(layer);

        let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some(&format!("Geometry Pass Layer {}", layer)),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: color_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::TRANSPARENT),
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &object_bind_group, &[]);
        rpass.draw(0..6, 0..objects.len() as u32);
    }
}
