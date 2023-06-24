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
            .with_inner_size(render::DEFAULT_WINDOW_SIZE)
            .with_title("Space Simulation")
            .build(&event_loop)
            .unwrap()
    };

    // rendering setup
    let window_size = window.inner_size();
    let mut renderer = FrameRenderer::new(
        window_size.width,
        window_size.height,
        SurfaceTexture::new(window_size.width, window_size.height, &window)
    );

    // simulation setup
    let mut simulation = SimulationContainer::new();
    simulation.add_actor(SimulationActor::new(200.0, 300.0, 100.0));
    simulation.add_actor(SimulationActor::new(600.0, 300.0, 100.0));
    simulation.render_me = false;
    simulation.suspend();

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
                        match _input.virtual_keycode.unwrap_or(VirtualKeyCode::End){
                            VirtualKeyCode::Period => { simulation.step(); },
                            VirtualKeyCode::Comma => {
                                renderer.clear_frame([0u8; 4]);
                                draw_sim_to_frame(&mut renderer, &simulation);
                            },
                            VirtualKeyCode::Key1 => simulation.resume(),
                            VirtualKeyCode::Key2 => simulation.suspend(),
                            VirtualKeyCode::Key3 => simulation.prune(),
                            VirtualKeyCode::Key0 => simulation.render_me = false,
                            VirtualKeyCode::Key9 => simulation.render_me = true,
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
            Event::Suspended => {},
            Event::Resumed => {},
            Event::MainEventsCleared => {
                renderer.clear_frame([0u8; 4]);
                if simulation.render_me{
                    const SIM_STEP: std::time::Duration = std::time::Duration::from_millis(50);
                    if simulation.is_running && simulation.prev_step.elapsed().unwrap() >= SIM_STEP {
                        simulation.step();
                    }
                    draw_sim_to_frame(&mut renderer, &simulation);
                }
                renderer.draw_line(50, 50, 250, 250, 1, [255u8; 4]);
                renderer.draw_line(100, 100, 100, 400, 2, [200u8; 4]);
                renderer.render();
            },
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
            if p.0 < 0.0 || p.1 < 0.0 { continue; }
            (
                if p.0 >= u32::MAX as f64 { continue; } else { p.0.round() as u32 },
                if p.1 >= u32::MAX as f64 { continue; } else { p.1.round() as u32 }
            )
        };
        renderer.draw_sphere(pos.0, pos.1, actor.radius(), actor.get_color());
    }
}
