use pixels::SurfaceTexture;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use simulation::SimulationContainer;
use crate::render::FrameRenderer;
use crate::simulation::SimulationActor;

mod simulation;
mod render;

fn main() {
    // window setup
    let event_loop = EventLoop::new();
    let window = {
        WindowBuilder::new()
            .with_inner_size(render::DEFAULT_SIZE)
            .with_title("Space Simulation")
            .build(&event_loop)
            .unwrap()
    };

    // rendering setup
    let window_size = window.inner_size();
    let mut renderer = render::FrameRenderer::new(
        window_size.width,
        window_size.height,
        SurfaceTexture::new(window_size.width, window_size.height, &window)
    );

    // simulation setup
    let mut simulation = SimulationContainer::new();
    simulation.add_actor(SimulationActor::new(200.0, 200.0, 100.0));
    simulation.add_actor(SimulationActor::new(400.0, 400.0, 100.0));

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
                    WindowEvent::Resized(_size) => renderer.resize(_size),
                    WindowEvent::DroppedFile(_) => {},
                    WindowEvent::KeyboardInput {input: _input,..} =>{
                        match _input.virtual_keycode.unwrap(){
                            VirtualKeyCode::Period => { simulation.step(); },
                            VirtualKeyCode::Comma => {
                                renderer.clear_frame([0u8; 4]);
                                draw_sim_to_frame(&mut renderer, &simulation);
                            },
                            _ => {},
                        }
                    },
                    WindowEvent::MouseInput {..}=> {},
                    WindowEvent::CursorMoved {..} => {},
                    _ => {}
                }
            },
            Event::DeviceEvent { .. } => {},
            Event::UserEvent(_) => {},
            Event::Suspended => simulation.suspend(),
            Event::Resumed => simulation.resume(),
            Event::MainEventsCleared => { renderer.render(); },
            Event::RedrawRequested(id) if id == window.id() => {},
            Event::RedrawEventsCleared => {},
            Event::LoopDestroyed => {},
            _ => {}
        }
    }
    );
}

fn draw_sim_to_frame(renderer: &mut FrameRenderer, sim: &SimulationContainer){
    for actor in &sim.space{
        let pos = {
            let p = actor.coordinates();
            (
                if p.0 >= u32::MAX as f64 { continue; } else { p.0.round() as u32 },
                if p.1 >= u32::MAX as f64 { continue; } else { p.1.round() as u32 }
            )
        };
        renderer.draw_sphere(pos.0, pos.1, actor.radius(), actor.get_color());
    }
}
