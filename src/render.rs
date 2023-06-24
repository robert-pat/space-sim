use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;

pub(crate) const DEFAULT_WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize{width: 800, height: 600};
const DEFAULT_LINE_THICKNESS: f64 = 1f64 / 1000f64;

pub struct FrameRenderer {
    current_frame: Pixels,
    size: PhysicalSize<u32>,
}
#[allow(unused)]
impl FrameRenderer {
    pub fn new(width: u32, height: u32, texture: SurfaceTexture<Window>) -> Self{
        Self {
            current_frame: Pixels::new(width, height, texture).unwrap(),
            size: PhysicalSize{width, height},
        }
    }
    pub fn render(&self){
        match self.current_frame.render(){
            Ok(_)=>{},
            Err(e) => {eprint!("Rendering Failed! {:?}", e);}
        };
    }
    pub fn clear_frame(&mut self, color: [u8; 4]){
        self.current_frame.frame_mut().chunks_exact_mut(4).for_each(
            |p|{p.copy_from_slice(&color);}
        );
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]){
        let pos = (x + y * self.size.width) as usize;
        self.current_frame.frame_mut()
            .chunks_exact_mut(4).nth(pos)
            .unwrap().copy_from_slice(&color);
    }
    pub fn checker_board(&mut self){
        let mut color = [13, 152, 186, 0];
        for pixel in self.current_frame.frame_mut().chunks_exact_mut(4){
            pixel.copy_from_slice(&color);
            for c in color.iter_mut(){ *c %= 255; *c += 1; }
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>){
        self.size = size;
        match self.current_frame.resize_surface(size.width, size.height){
            Ok(_) => {},
            Err(e) => eprintln!("Surface Resize Failed: {}", e),
        }
        match self.current_frame.resize_buffer(size.width, size.height){
            Ok(_) => {},
            Err(e) => eprintln!("Buffer Resize Failed: {}", e),
        }
    }
    pub fn num_pixels(&self) -> u64{ self.size.width as u64 * self.size.height as u64 }
    pub fn dimensions(&self) -> PhysicalSize<u32>{ self.size }
    pub fn draw_sphere(&mut self, center_x: u32, center_y: u32, radius: u32, color: [u8; 4]){
        for (i, p) in
        self.current_frame.frame_mut().chunks_exact_mut(4).enumerate(){
            let pos = to_pixel_coordinates(i, &self.size);
            let dx = pos.0 as f64 - center_x as f64; // cast both b/c u32 would underflow
            let dy = pos.1 as f64 - center_y as f64;
            if (dx.powf(2f64) + dy.powf(2f64)).sqrt() <= radius as f64{
                p.copy_from_slice(&color);
            }
        }
    }
    pub fn draw_rectangle(&mut self, corner_x: u32, corner_y: u32, width: u32, height: u32, color: [u8; 4]){
        for (i, p) in
        self.current_frame.frame_mut().chunks_exact_mut(4).enumerate(){
            let pos = to_pixel_coordinates(i, &self.size);
            if pos.0  >= corner_x && pos.0 <= corner_x + width && pos.1 >= corner_y &&
                pos.1 <= corner_y + height {
                p.copy_from_slice(&color);
            }
        }
    }
    pub fn draw_line(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32, width: u8, color: [u8; 4]){
        // TODO: this doesn't work at all
        let m = (end_y as f64 - start_y as f64) / (end_x as f64 - start_x as f64);
        let b = start_y as f64 - m * start_x as f64;
        let w = DEFAULT_LINE_THICKNESS * width as f64;
        for (i, p ) in
        self.current_frame.frame_mut().chunks_exact_mut(4).enumerate(){
            let pos = to_pixel_coordinates(i, &self.size);
            // from y = mx + b -> mx + b - y = 0 for all points on the line
            // line width controls how close to the "true" line (0 thickness)
            if ((m * pos.0 as f64) + b - pos.1 as f64).abs() < w {
                p.copy_from_slice(&color);
            }
        }
    }
    pub fn to_pixel(&self, pos: PhysicalPosition<f64>) -> (u32, u32){
        match self.current_frame.window_pos_to_pixel(pos.into()){
            Ok(t) => (t.0 as u32, t.1 as u32),
            Err(_) => (0u32, 0u32),
        }
    }
}
#[allow(unused)]
fn to_index(x: u32, y: u32, size: &PhysicalSize<u32>)-> usize{
    (x + y * size.width) as usize
}
#[allow(unused)]
fn to_pixel_coordinates(index: usize, size: &PhysicalSize<u32>) -> (u32, u32){
    (index as u32 % size.width, index as u32 / size.width)
}