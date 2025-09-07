mod app;
mod renderer;

fn main() {
    let mut app = app::App::new();
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
