

fn main() {
    let vsync = false;

    let updates_per_second = 20;

    let mut time_of_last_update = std::time::Instant::now();
    let static_update_pause = std::time::Duration::from_millis(1000 / updates_per_second);
    let mut update_pause = static_update_pause;

    let renders_per_second = 60;

    let mut time_of_last_render = std::time::Instant::now();
    let static_render_pause = std::time::Duration::from_millis(1000 / renders_per_second);
    let mut render_pause = static_update_pause;

    let event_loop = winit::event_loop::EventLoop::new();

    let _window = winit::window::Window::new(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(_) => {
                    // TODO resize surface
                }
                winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                _ => (),
            }
            winit::event::Event::MainEventsCleared => {
                if time_of_last_update.elapsed() >= update_pause {
                    time_of_last_update = std::time::Instant::now();
                    update();
                    update_pause = match static_update_pause.checked_sub(time_of_last_update.elapsed()) {
                        Some(d) => d,
                        None => std::time::Duration::ZERO,
                    };
                }

                if vsync {
                    render();
                } else if time_of_last_render.elapsed() >= render_pause {
                    time_of_last_render = std::time::Instant::now();
                    render();
                    render_pause = match static_render_pause.checked_sub(time_of_last_render.elapsed()) {
                        Some(d) => d,
                        None => std::time::Duration::ZERO,
                    };
                }
            }
            _ => (),
        }
    });
}

fn update() {}

fn render() {}
