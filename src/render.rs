use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::window::Window;

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
    pub fn resize(&mut self, size: PhysicalSize<u32>){
        match self.current_frame.resize_surface(size.width, size.height){
            Ok(_) => {},
            Err(e) => {eprintln!("Resizing Failed! {:?}", e);}
        };
    }
}