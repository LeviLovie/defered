use bevy_ecs::prelude::*;
use bevy_ecs::{schedule::Schedule, world::World};
use nalgebra::Vector2;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::Key,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Window, WindowId},
};

use crate::renderer::ecs::{Texture, Transform};
use crate::renderer::{
    camera::Camera,
    ecs::{process_ecs, renderer::RendererRes},
    Renderer,
};

const TARGET_FPS: f32 = 60.0;

pub fn init_object(mut commands: Commands) {
    for i in 0..16 {
        commands.spawn((
            Transform {
                position: Vector2::new(10.0, 10.0 + i as f32 * 50.0),
                scale: Vector2::new(3.0, 3.0),
                rotation: 180.0,
                layer: i % 4,
            },
            Texture {
                handle: None,
                path: "rocket.png".to_string(),
                width: 32,
                height: 32,
            },
        ));
    }
}

pub struct App {
    renderer: Option<Arc<Mutex<Renderer>>>,
    last_frame: Instant,
    frame_time: Duration,
    accumulator: Duration,
    camera: Camera,
    world: World,
    update: Schedule,
    draw: Schedule,
    process: Schedule,
}

impl App {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut init = Schedule::default();
        init.add_systems(init_object);
        init.run(&mut world);

        let update = Schedule::default();
        let draw = Schedule::default();
        let process = process_ecs();

        Self {
            renderer: None,
            camera: Camera::new([0.0, 0.0], [800.0, 600.0], 50.0),
            last_frame: Instant::now(),
            frame_time: Duration::from_secs_f32(1.0 / TARGET_FPS),
            accumulator: Duration::ZERO,
            world,
            update,
            draw,
            process,
        }
    }

    pub fn update(&mut self) {
        self.update.run(&mut self.world);
    }

    pub fn draw(&mut self) {
        self.draw.run(&mut self.world);

        if let Some(renderer) = &self.renderer {
            self.world.insert_resource(RendererRes(renderer.clone()));
            self.process.run(&mut self.world);

            renderer.lock().unwrap().render(&self.camera);
        }
    }

    pub fn input(&mut self, event: KeyEvent) {
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
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Defered rendering"))
            .expect("Failed to create window");
        window.request_redraw();

        self.renderer = Some(Arc::new(Mutex::new(pollster::block_on(Renderer::new(
            Arc::new(window),
        )))));
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = now - self.last_frame;
        self.last_frame = now;
        self.accumulator += dt;

        while self.accumulator >= self.frame_time {
            self.accumulator -= self.frame_time;
            self.update();
            if let Some(renderer) = &self.renderer {
                renderer.lock().unwrap().window.request_redraw();
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
            if renderer.lock().unwrap().window.id() != window_id {
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.renderer
                    .as_mut()
                    .map(|r| r.lock().unwrap().resize(new_size.width, new_size.height));
            }
            WindowEvent::KeyboardInput { event, .. } => self.input(event),
            WindowEvent::RedrawRequested => self.draw(),
            _ => {}
        }
    }
}
