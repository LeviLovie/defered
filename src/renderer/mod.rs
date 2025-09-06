mod device;
mod gbuffer;
pub mod object;
mod passes;

use std::sync::Arc;
use object::Object;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

use gbuffer::GBuffer;

const RENDER_STATE: passes::RenderState = passes::RenderState::GBuffer;

pub struct Renderer {
    pub window: Arc<Window>,
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    gbuffer: GBuffer,
    geometry_pass: passes::Geometry,
    composite_pass: passes::Composite,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let (device, queue, surface, config) = device::init_wgpu(window.clone()).await;

        let gbuffer = GBuffer::new(&device, config.width, config.height);
        let geometry_pass = passes::Geometry::new(&device, &gbuffer);
        let composite_pass = passes::Composite::new(&device, config.format, &gbuffer, RENDER_STATE);

        Self {
            window,
            device,
            queue,
            surface,
            config,
            gbuffer,
            geometry_pass,
            composite_pass,
        }
    }

    pub fn render(&mut self, objects: Vec<Object>) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            self.geometry_pass
                .execute(&mut encoder, &self.gbuffer, &objects, &self.device);
        }
        {
            self.composite_pass
                .execute(&mut encoder, &view);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);

        self.gbuffer = GBuffer::new(&self.device, width, height);
        self.geometry_pass = passes::Geometry::new(&self.device, &self.gbuffer);
        self.composite_pass = passes::Composite::new(
            &self.device,
            self.config.format,
            &self.gbuffer,
            RENDER_STATE,
        );
    }
}
