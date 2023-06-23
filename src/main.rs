use pixels::SurfaceTexture;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use simulation::SimulationContainer;
use crate::simulation::SimulationActor;

mod simulation;
mod render;

fn main() {
    // window setup
    let event_loop = EventLoop::new();
    let mut window_build = WindowBuilder::new();
    window_build = window_build.with_inner_size(render::DEFAULT_SIZE);
    let window = window_build.build(&event_loop).unwrap();

    // rendering setup
    let size = window.inner_size();
    let texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut renderer = render::SimulationRenderer::new(
        size.width,
        size.height,
        texture
    );
    renderer.resize_display(PhysicalSize{width: 1600, height: 1200});

    // simulation setup
    let mut simulation = SimulationContainer::new();
    simulation.add_actor(SimulationActor::new(20.0, 20.0, 1.0));

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
                    WindowEvent::Resized(_size) => renderer.resize_display(_size),
                    WindowEvent::DroppedFile(_) => {},
                    WindowEvent::KeyboardInput {input: _input,..} =>{},
                    WindowEvent::MouseInput {..}=> {},
                    _ => {}
                }
            },
            Event::DeviceEvent { .. } => {},
            Event::UserEvent(_) => {},
            Event::Suspended => simulation.suspend(),
            Event::Resumed => simulation.resume(),
            Event::MainEventsCleared => {},
            Event::RedrawRequested(id) if id == window.id() => {},
            Event::RedrawEventsCleared => {},
            Event::LoopDestroyed => {},
            _ => {}
        }
    }
    );
}
