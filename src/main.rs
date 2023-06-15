use pixels::SurfaceTexture;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use simulation::SimulationContainer;

mod simulation;
mod render;

fn main() {
    // window setup
    let event_loop = EventLoop::new();
    let window_build = WindowBuilder::new();
    let window = window_build.build(&event_loop).unwrap();

    // rendering setup
    let size = window.inner_size();
    let texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut _renderer = render::SimulationRenderer::new(
        size.width,
        size.height,
        texture
    );

    // simulation setup
    let mut simulation = SimulationContainer::new();

    // build the closure to handle events in the loop & start it
    event_loop.run(
    move |event, _, control_flow| {
        control_flow.set_poll();
        match event{
            Event::NewEvents(_) => {},
            Event::WindowEvent{window_id, event} if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Focused(_b) => {},
                    WindowEvent::Resized(_size) => {},
                    WindowEvent::DroppedFile(path) => {
                        simulation.load_from_file(path.to_str().unwrap_or(""));
                    },
                    WindowEvent::KeyboardInput {input: _input,..} =>{},
                    WindowEvent::MouseInput {..}=> {},
                    _ => {}
                }
            }
            Event::DeviceEvent { .. } => {},
            Event::UserEvent(_) => {},
            Event::Suspended => {simulation.suspend();},
            Event::Resumed => {simulation.resume();},
            Event::MainEventsCleared => {
                _renderer.fill_color(_renderer.color_count);
                _renderer.render();
                let c = _renderer.color_count;
                if c <= 254{
                    _renderer.color_count += 1;
                }
                else{
                    _renderer.color_count = 0;
                }
            }, // either render here
            Event::RedrawRequested(id) if id == window.id() => {}, // or render here
            Event::RedrawEventsCleared => {},
            Event::LoopDestroyed => {},
            _ => {}
        }
    }
    );
}
