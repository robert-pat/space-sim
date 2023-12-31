use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;
pub(crate) const DEFAULT_WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize {
    width: 800,
    height: 600,
};
const LINE_TOLERANCE: f64 = std::f64::consts::SQRT_2;

pub struct FrameRenderer {
    current_frame: Pixels,
    size: PhysicalSize<u32>,
}
#[derive(Copy, Clone, Debug)]
pub struct PixelPoint(pub u32, pub u32);

#[allow(unused)]
impl FrameRenderer {
    pub fn new(width: u32, height: u32, texture: SurfaceTexture<Window>) -> Self {
        Self {
            current_frame: Pixels::new(width, height, texture).unwrap(),
            size: PhysicalSize { width, height },
        }
    }
    pub fn render(&self) {
        match self.current_frame.render() {
            Ok(_) => {}
            Err(e) => {
                eprint!("Rendering Failed! {:?}", e);
            }
        };
    }
    pub fn clear_frame(&mut self, color: [u8; 4]) {
        self.current_frame
            .frame_mut()
            .chunks_exact_mut(4)
            .for_each(|p| {
                p.copy_from_slice(&color);
            });
    }
    pub fn set_pixel(&mut self, pos: PixelPoint, color: [u8; 4]) {
        let pos = (pos.0 + pos.1 * self.size.width) as usize;
        self.current_frame
            .frame_mut()
            .chunks_exact_mut(4)
            .nth(pos)
            .unwrap()
            .copy_from_slice(&color);
    }
    pub fn checker_board(&mut self) {
        let mut color = [13, 152, 186, 0];
        for pixel in self.current_frame.frame_mut().chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
            for c in color.iter_mut() {
                *c %= 255;
                *c += 1;
            }
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        match self.current_frame.resize_surface(size.width, size.height) {
            Ok(_) => {}
            Err(e) => eprintln!("Surface Resize Failed: {}", e),
        }
        match self.current_frame.resize_buffer(size.width, size.height) {
            Ok(_) => {}
            Err(e) => eprintln!("Buffer Resize Failed: {}", e),
        }
    }
    pub fn num_pixels(&self) -> u64 {
        self.size.width as u64 * self.size.height as u64
    }
    pub fn dimensions(&self) -> PhysicalSize<u32> {
        self.size
    }
    pub fn draw_sphere(&mut self, center: PixelPoint, radius: u32, color: [u8; 4]) {
        for (i, p) in self
            .current_frame
            .frame_mut()
            .chunks_exact_mut(4)
            .enumerate()
        {
            let pos = to_pixel_coordinates(i, &self.size);
            let dx = pos.0 as f64 - center.0 as f64; // cast both b/c u32 would underflow
            let dy = pos.1 as f64 - center.1 as f64;
            if (dx.powf(2f64) + dy.powf(2f64)).sqrt() <= radius as f64 {
                p.copy_from_slice(&color);
            }
        }
    }
    pub fn draw_rectangle(&mut self, corner: PixelPoint, width: u32, height: u32, color: [u8; 4]) {
        for (i, p) in self
            .current_frame
            .frame_mut()
            .chunks_exact_mut(4)
            .enumerate()
        {
            let pos = to_pixel_coordinates(i, &self.size);
            if pos.0 >= corner.0
                && pos.0 <= corner.0 + width
                && pos.1 >= corner.1
                && pos.1 <= corner.1 + height
            {
                p.copy_from_slice(&color);
            }
        }
    }
    pub fn draw_line(&mut self, start: PixelPoint, end: PixelPoint, color: [u8; 4]) {
        // TODO: consider modifying this to take a line weight (int multiple of LINE_TOLERANCE) ?
        let m = (end.1 as f64 - start.1 as f64) / (end.0 as f64 - start.0 as f64);
        let b = m * start.0 as f64 - start.1 as f64;
        let x_range = start.0..=end.0;
        let y_range = start.1..=end.1;

        for (i, p) in self
            .current_frame
            .frame_mut()
            .chunks_exact_mut(4)
            .enumerate()
        {
            let pos = to_pixel_coordinates(i, &self.size);
            if !x_range.contains(&pos.0) || !y_range.contains(&pos.1) {
                continue;
            }
            let calc = m * pos.0 as f64 - pos.1 as f64;
            if calc >= b - LINE_TOLERANCE && calc <= b + LINE_TOLERANCE {
                p.copy_from_slice(&color);
            }
        }
    }
    // From: https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    // Doesn't currently work with sucks + idk how to get it working :(
    #[warn(deprecated)] // TODO: make pub(crate) once this starts working
    fn draw_triangle(&mut self, vertex: (PixelPoint, PixelPoint, PixelPoint), color: [u8; 4]) {
        fn sign_test_triangle(v1: PixelPoint, v2: PixelPoint, v3: PixelPoint) -> i32 {
            let (v1_x, v1_y) = (v1.0 as i32, v1.1 as i32);
            let (v2_x, v2_y) = (v2.0 as i32, v1.1 as i32);
            let (v3_x, v3_y) = (v3.0 as i32, v3.1 as i32);
            (v1_x - v3_x) * (v2_y - v3_y) - (v2_x - v3_x) * (v1_y - v3_y)
        }
        for (index, pixel) in self
            .current_frame
            .frame_mut()
            .chunks_exact_mut(4)
            .enumerate()
        {
            let point = to_pixel_coordinates(index, &self.size);
            let (d1, d2, d3) = (
                sign_test_triangle(point, vertex.0, vertex.1),
                sign_test_triangle(point, vertex.1, vertex.2),
                sign_test_triangle(point, vertex.2, vertex.1),
            );

            let h_neg = (d1 < 0) || (d2 < 0) || (d3 < 0);
            let h_pos = (d1 > 0) || (d2 > 0) || (d3 > 0);

            if !(h_neg && h_pos) {
                pixel.copy_from_slice(&color);
            }
        }
    }
    pub fn window_to_pixel(&self, pos: PhysicalPosition<f64>) -> PixelPoint {
        match self.current_frame.window_pos_to_pixel(pos.into()) {
            Ok(t) => PixelPoint(t.0 as u32, t.1 as u32),
            Err(_) => PixelPoint(0u32, 0u32),
        }
    }
}
#[allow(unused)]
fn to_index(p: PixelPoint, size: &PhysicalSize<u32>) -> usize {
    (p.0 + p.1 * size.width) as usize
}
#[allow(unused)]
fn to_pixel_coordinates(index: usize, size: &PhysicalSize<u32>) -> PixelPoint {
    PixelPoint(index as u32 % size.width, index as u32 / size.width)
}
pub fn draw_sim_to_frame(
    renderer: &mut FrameRenderer,
    sim: &crate::simulation::SimulationContainer,
) {
    for actor in &sim.space {
        let pos = {
            let p = actor.coordinates();
            if p.0 < 0.0 || p.1 < 0.0 {
                continue;
            }
            PixelPoint(
                if p.0 >= u32::MAX as f64 {
                    continue;
                } else {
                    p.0.round() as u32
                },
                if p.1 >= u32::MAX as f64 {
                    continue;
                } else {
                    p.1.round() as u32
                },
            )
        };
        renderer.draw_sphere(pos, actor.radius(), actor.get_color());
    }
}

pub fn showcase_shapes(r: &mut FrameRenderer) {
    r.draw_line(
        PixelPoint(183, 291),
        PixelPoint(670, 415),
        [0, 128, 128, 255],
    );
    r.draw_sphere(PixelPoint(170, 170), 95, [200, 50, 90, 255]);
    r.draw_rectangle(PixelPoint(400, 400), 75, 150, [0, 255, 0, 255]);
}
