mod renderer;

use std::sync::Arc;

use renderer::{Renderer, object::Object};
use winit::{
    application::ApplicationHandler,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

struct App {
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let renderer = pollster::block_on(Renderer::new(window.clone()));
        self.renderer = Some(renderer);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let Some(renderer) = &self.renderer {
            if renderer.window.id() != window_id {
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let objects = vec![
                    Object {
                        pos: [10.0, 10.0],
                        size: [20.0, 20.0],
                        layer: 0,
                        obj_type: 0,
                        special_data: [0; 4],
                    },
                    Object {
                        pos: [15.0, 15.0],
                        size: [20.0, 20.0],
                        layer: 1,
                        obj_type: 1,
                        special_data: [0; 4],
                    },
                ];

                if let Some(renderer) = &mut self.renderer {
                    renderer.render(objects);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(renderer) = &self.renderer {
            renderer.window.request_redraw();
        }
    }
}

fn main() {
    let mut app = App { renderer: None };
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
