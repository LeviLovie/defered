mod passes;
mod device;
mod gbuffer;

use std::sync::Arc;

use winit::window::Window;

use gbuffer::GBuffer;

pub struct Renderer {
    pub window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    gbuffer: GBuffer,
    geometry_pass: passes::Geometry,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let (device, queue, surface, config) = device::init_wgpu(window.clone()).await;

        let gbuffer = GBuffer::new(&device, config.width, config.height);
        let geometry_pass = passes::Geometry::new(&device, &gbuffer);

        Self {
            window,
            device,
            queue,
            surface,
            config,
            gbuffer,
            geometry_pass,
        }
    }

    pub fn render(&mut self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        self.geometry_pass.execute(&mut encoder, &self.gbuffer, &view);

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
    }
}
