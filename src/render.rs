use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::simulation;

pub(crate) const DEFAULT_SIZE: PhysicalSize<u32> = PhysicalSize{width: 400, height: 300};

pub struct SimulationRenderer{
    current_frame: Pixels,
    size: PhysicalSize<u32>,
    x_min: i32,
    y_min: i32,
}
impl SimulationRenderer{
    pub fn new(width: u32, height: u32, texture: SurfaceTexture<Window>) -> Self{
        Self {
           current_frame: Pixels::new(width, height, texture).unwrap(),
           size: PhysicalSize{width, height},
            x_min: 0i32,
            y_min: 0i32,
        }
    }
    pub fn render(&self){
        match self.current_frame.render(){
            Ok(_)=>{},
            Err(e) => {eprint!("Rendering Failed! {:?}", e);}
        };
    }
    pub fn clear_frame(&mut self, color: u8){
        self.current_frame.frame_mut().iter_mut().for_each(|x| {*x = color});
    }
    pub fn set_pixel(&mut self, pixel: usize, color: u8){
        *self.current_frame.frame_mut().iter_mut().nth(pixel).unwrap() = color;
    }
    pub fn resize_display(&mut self, size: PhysicalSize<u32>){
        match self.current_frame.resize_surface(size.width, size.height){
            Ok(_) => {},
            Err(e) => {eprintln!("Resizing Failed! {:?}", e);}
        };
    }
    pub fn change_resolution(&mut self, size: PhysicalSize<u32>){
        self.size = size;
        match self.current_frame.resize_buffer(size.width, size.height){
            Ok(_) => {},
            Err(e) => eprintln!("Resizing Failed! {:?}", e)
        }
    }
    pub fn move_view(&mut self, x: i32, y: i32){
        self.x_min = x;
        self.y_min = y;
    }
    pub fn display_simulation(&mut self, sim: &simulation::SimulationContainer){
        self.clear_frame(0u8);
        let frame = self.current_frame.frame_mut();

        for s in &sim.space{
            // convert f64 math pos -> pixel pos -> array index
            let c = s.get_coordinates();
            let pos = ( // convert f64 -> i32 w/o panics & skip objects out of range
                if c.0 >= i32::max_value() as f64 {continue;}
                else{c.0.round() as i32 - self.x_min},
                if c.1 >= i32::max_value() as f64 {continue;}
                else{c.1.round() as i32 - self.y_min}
            );
            let index = pos.1 + pos.0 * self.size.width as i32; // this feels like bad
            frame[index as usize] = s.get_color();
        }
        self.render();
    }
}