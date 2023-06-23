use std::thread::sleep;
use itertools::Itertools;

const GRAVITATIONAL_CONSTANT: f64 = 1.0;
const SIMULATION_TIMESTEP: f64 = 0.05;

#[derive(Clone)]
pub struct SimulationActor{
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    mass: f64,
    radius: f64,
    color: [u8; 3]
}
impl SimulationActor{
    pub fn get_coordinates(&self) -> (f64, f64){
        (self.x_pos, self.y_pos)
    }
}
impl Default for SimulationActor{
    fn default() -> Self {
        Self{
            x_pos: 0.0,
            y_pos: 0.0,
            x_vel: 0.0,
            y_vel: 0.0,
            mass: 1.0,
            radius: 1.0,
            color: [0, 0, 0],
        }
    }
}

pub struct SimulationContainer{
    pub(crate) is_running: bool,
    pub(crate) space: Vec<SimulationActor>,
    x_max: usize,
    y_max: usize,
}
impl SimulationContainer{
    pub fn new(x: usize, y: usize) -> Self{
        Self{
            is_running: false,
            space: vec![SimulationActor::default(); 20],
            x_max: x,
            y_max: y,
        }
    }
    pub fn suspend(&mut self){
        self.is_running = false;
    }
    pub fn resume(&mut self){
        self.is_running = true;
    }
    pub fn load_from_file(&mut self, path: &str)->Self{
        if path.is_empty(){
            SimulationContainer::default()
        }
        else{
            todo!()
        }
    }
    pub fn step(&mut self){
        for inner in 0..self.space.len() {
            for outer in 0..self.space.len() {
                if inner == outer{continue;}
                apply_gravity(&mut self.space, inner, outer);
            }
        }
        move_actors(&mut self.space);
    }
}
impl Default for SimulationContainer{
    fn default() -> Self {
        SimulationContainer::new(crate::DEFAULT_X, crate::DEFAULT_Y)
    }
}

pub fn health_check(act: &SimulationActor) ->bool{
    if act.radius <= 1.0{
        false
    }
    else { act.mass > 0.0 }
}
pub fn apply_gravity(elements: &mut [SimulationActor], first: usize, second: usize){
    let f = &elements[first];
    let s = &elements[second];

    let dx = (f.x_pos - s.x_pos).abs();
    let dy = (f.y_pos - s.y_pos).abs();
    let r = (dx.powf(2f64) + dy.powf(2f64)).sqrt();
    let mag = GRAVITATIONAL_CONSTANT * f.mass * s.mass * r.recip().powf(2f64);
    let theta = (dy / dx).atan();

    elements[first].x_vel += mag * theta.cos();
    elements[first].y_vel += mag * theta.sin();

    elements[second].x_vel += (-mag) * theta.cos(); //TODO: test this math
    elements[second].y_vel += (-mag) * theta.sin();
}
pub fn move_actors(actors: &mut [SimulationActor]){
    for a in actors.iter_mut(){
        a.x_pos += a.x_vel * SIMULATION_TIMESTEP;
        a.y_pos += a.y_vel * SIMULATION_TIMESTEP;
    }
}