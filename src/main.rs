mod renderer;

use std::sync::Arc;

use renderer::{object::Object, Renderer};
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
                    vec![Object {
                        pos: [100.0, 100.0],
                        size: [250.0, 25.0],
                        color: [0.0, 1.0, 0.0, 1.0],
                    }],
                    vec![
                        Object {
                            pos: [200.0, 200.0],
                            size: [200.0, 200.0],
                            color: [1.0, 0.0, 0.0, 1.0],
                        },
                        Object {
                            pos: [500.0, 400.0],
                            size: [300.0, 500.0],
                            color: [0.0, 0.0, 1.0, 1.0],
                        },
                    ],
                    vec![Object {
                        pos: [750.0, 600.0],
                        size: [500.0, 50.0],
                        color: [1.0, 1.0, 1.0, 1.0],
                    }],
                    vec![Object {
                        pos: [700.0, 650.0],
                        size: [20.0, 50.0],
                        color: [1.0, 0.0, 1.0, 1.0],
                    }],
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
