use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, Color,
    ColorTargetState, ColorWrites, CompareFunction, DepthStencilState, Device, FragmentState,
    LoadOp, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderStages, StoreOp, VertexState,
};

use crate::renderer::{camera::Camera, gbuffer::GBuffer, object::Object};

use super::RenderPassData;

pub struct Geometry {
    pipeline: RenderPipeline,
    objects_bgl: BindGroupLayout,
    params_bg: BindGroup,
    frame_b: Buffer,
    camera_b: Buffer,
}

impl Geometry {
    pub fn new(device: &Device, gbuffer: &GBuffer) -> Self {
        let shader = device.create_shader_module(include_wgsl!("../../shaders/geometry.wgsl"));

        let objects_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let frame = gbuffer.frame();
        let frame_b = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Frame B"),
            contents: bytemuck::cast_slice(&[frame]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera = Camera::default();
        let camera_b = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera B"),
            contents: bytemuck::cast_slice(&[camera]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let params_bgl = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Params BGL"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let params_bg = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Params BG"),
            layout: &params_bgl,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: frame_b.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: camera_b.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Geometry Pipeline Layout"),
            bind_group_layouts: &[&objects_bgl, &params_bgl],
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
            objects_bgl,
            params_bg,
            frame_b,
            camera_b,
        }
    }

    pub fn execute(
        &self,
        data: &mut RenderPassData,
        objects: &[Object],
        layer: u32,
        camera: &Camera,
    ) {
        data.queue
            .write_buffer(&self.camera_b, 0, bytemuck::cast_slice(&[*camera]));
        let frame = data.gbuffer.frame();
        data.queue
            .write_buffer(&self.frame_b, 0, bytemuck::cast_slice(&[frame]));

        let objects_b = data.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Object Buffer"),
            contents: bytemuck::cast_slice(objects),
            usage: BufferUsages::STORAGE,
        });
        let objects_bg = data.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Object Bind Group"),
            layout: &self.objects_bgl,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: objects_b.as_entire_binding(),
            }],
        });

        let color_view = &data.gbuffer.color_layer_view(layer);
        let depth_view = &data.gbuffer.depth_layer_view(layer);

        let mut rpass = data.encoder.begin_render_pass(&RenderPassDescriptor {
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
        rpass.set_bind_group(0, &objects_bg, &[]);
        rpass.set_bind_group(1, &self.params_bg, &[]);
        rpass.draw(0..6, 0..objects.len() as u32);
    }
}
