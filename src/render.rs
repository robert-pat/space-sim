use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::simulation;

pub struct SimulationRenderer{
    current_frame: Pixels,
    pub color_count: u8
}
impl SimulationRenderer{
    pub fn new(width: u32, height: u32, texture: SurfaceTexture<Window>) -> Self{
        Self {
           current_frame: Pixels::new(width, height, texture).unwrap(),
           color_count: 0_u8,
        }
    }
    pub fn render(&self){
        match self.current_frame.render(){
            Ok(_)=>{},
            Err(e) => {eprint!("Rendering Failed! {:?}", e);}
        };
    }
    pub fn fill_color(&mut self, color: u8){
        for p in self.current_frame.frame_mut(){
            *p = color;
        }
    }
    // Only want to resize the thing displaying the simulation, the actual sim size should not change
    pub fn resize(&mut self, size: PhysicalSize<u32>){
        match self.current_frame.resize_surface(size.width, size.height){
            Ok(_) => {},
            Err(e) => {eprintln!("Resizing Failed! {:?}", e);}
        };
    }
    pub fn display_simulation(&mut self, sim: &simulation::SimulationContainer){
        self.fill_color(0u8);
        let frame = self.current_frame.frame_mut();
        for s in &sim.space{
            let coords = s.get_coordinates();
            let position_in_frame =
                coords.1.round() as usize /*x dimension of the window*/ + coords.0.round() as usize;
            frame[position_in_frame] = 255u8;
        }
        self.render();
    }
}