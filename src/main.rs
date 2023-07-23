use pixels::SurfaceTexture;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use simulation::SimulationContainer;
use crate::render::FrameRenderer;
use crate::simulation::SimulationActor;

mod simulation;
mod render;

const SIM_STEP: std::time::Duration = std::time::Duration::from_millis(50);

enum DisplayMode{
    Simulation,
    Showcase
}

/*Probably need to create a struct to hold which things need to be rendered.
* e.g. which DisplayMode, whether or not specific overlays need to be shown, etc. */

fn main() {
    let event_loop = EventLoop::new();
    let window = {
        WindowBuilder::new()
            .with_inner_size(render::DEFAULT_WINDOW_SIZE)
            .with_title("Space Simulation")
            .with_window_icon(None) //TODO: add an icon
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
    let mut current_render_mode = DisplayMode::Showcase;

    // simulation setup
    let mut simulation = SimulationContainer::new();
    simulation.add_actor(SimulationActor::new(200.0, 300.0, 100.0, [255u8; 4]));
    simulation.add_actor(SimulationActor::new(600.0, 300.0, 100.0, [128u8; 4]));
    simulation.suspend();

    // building the closure to handle control:
    // user control -> Event::WindowEvent
    // what to display -> Event::MainEventsCleared
    event_loop.run(
    move |event, _, control_flow| {
        control_flow.set_poll();
        match event{
            Event::NewEvents(_) => {},
            Event::WindowEvent{window_id, event} if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::Resized(_size) => renderer.resize(_size),
                    WindowEvent::KeyboardInput {input: _input,..} =>{
                        // Keyboard controls go here v
                        match _input.virtual_keycode.unwrap_or(VirtualKeyCode::End){
                            VirtualKeyCode::Key1 => current_render_mode = DisplayMode::Showcase,
                            VirtualKeyCode::Key2 => current_render_mode = DisplayMode::Simulation,
                            VirtualKeyCode::Key3 => simulation.step(),
                            VirtualKeyCode::Key4 => simulation.resume(),
                            VirtualKeyCode::Key5 => simulation.suspend(),
                            _ => {},
                        }
                    },
                    WindowEvent::MouseInput {..}=> {},
                    WindowEvent::CursorMoved {..} => {},
                    _ => {}
                }
            },
            Event::MainEventsCleared => {
                renderer.clear_frame([0u8; 4]);
                match current_render_mode{
                    DisplayMode::Showcase => render::showcase_shapes(&mut renderer),
                    DisplayMode::Simulation => {
                        if simulation.is_running && simulation.prev_step.elapsed().unwrap() >= SIM_STEP {
                            simulation.step();
                        }
                        render::draw_sim_to_frame(&mut renderer, &simulation);
                    }
                }
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