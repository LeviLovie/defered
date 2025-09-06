mod renderer;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::Key,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowId},
};

const TARGET_FPS: f32 = 60.0;

use renderer::{camera::Camera, object::Object, Renderer};
struct App {
    renderer: Option<Renderer>,
    last_frame: Instant,
    frame_time: Duration,
    camera: Camera,
}

impl Default for App {
    fn default() -> Self {
        Self {
            renderer: None,
            camera: Camera::new([0.0, 0.0], [800.0, 600.0]),
            last_frame: Instant::now(),
            frame_time: Duration::from_secs_f32(1.0 / TARGET_FPS),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Window::default_attributes().with_title("Defered rendering");

        let window = Arc::new(event_loop.create_window(window).unwrap());

        let renderer = pollster::block_on(Renderer::new(window.clone()));
        self.renderer = Some(renderer);

        window.request_redraw();
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let now = Instant::now();
        if now - self.last_frame >= self.frame_time {
            self.last_frame = now;
            if let Some(renderer) = &self.renderer {
                renderer.window.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
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
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match event.key_without_modifiers().as_ref() {
                        Key::Character("w") => {
                            self.camera.pos[1] += 20.0;
                        }
                        Key::Character("s") => {
                            self.camera.pos[1] -= 20.0;
                        }
                        Key::Character("a") => {
                            self.camera.pos[0] -= 20.0;
                        }
                        Key::Character("d") => {
                            self.camera.pos[0] += 20.0;
                        }
                        Key::Character("q") => {
                            self.camera.size[0] *= 1.05;
                            self.camera.size[1] *= 1.05;
                        }
                        Key::Character("e") => {
                            self.camera.size[0] *= 0.95;
                            self.camera.size[1] *= 0.95;
                        }
                        _ => (),
                    }
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
                    renderer.render(objects, &self.camera);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut app = App::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
