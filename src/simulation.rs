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
    pub fn new(x: f64, y: f64, mass: f64) -> Self {
        Self{
            x_pos: x,
            y_pos: y,
            x_vel: 0.0,
            y_vel: 0.0,
            mass,
            radius: 0.0,
            color: [0, 0, 0],
        }
    }
    pub fn get_coordinates(&self) -> (f64, f64){
        (self.x_pos, self.y_pos)
    }
    pub fn get_color(&self) -> u8{
        // TODO: this should actually do something
        0u8
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
    pub(crate) prev_step: std::time::SystemTime,
}
impl SimulationContainer{
    pub fn new() -> Self{
        Self{
            is_running: false,
            space: vec![SimulationActor::default(); 20],
            prev_step: std::time::SystemTime::now(),
        }
    }
    pub fn suspend(&mut self){
        self.is_running = false;
    }
    pub fn resume(&mut self){
        self.is_running = true;
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
    pub fn add_actor(&mut self, a: SimulationActor){
        self.space.push(a);
    }
    pub fn prune(&mut self){
        let mut v: Vec<usize> = Vec::new();
        for actor in self.space.iter(){
            //TODO: check if the item is ridiculously far away, really small or large mass, ..
        }
        for index in v.into_iter().rev(){
            self.space.remove(index);
        }
    }
}
impl Default for SimulationContainer{
    fn default() -> Self {
        SimulationContainer::new()
    }
}

pub fn apply_gravity(elements: &mut [SimulationActor], first: usize, second: usize){
    let f = &elements[first];
    let s = &elements[second];

    let dx = f.x_pos - s.x_pos;
    let dy = f.y_pos - s.y_pos;
    let r = (dx.powf(2f64) + dy.powf(2f64)).sqrt();
    let mag = GRAVITATIONAL_CONSTANT * f.mass * s.mass * r.recip().powf(2f64);
    let theta = (dy / dx).atan();

    elements[first].x_vel += mag * theta.cos() * -dx.signum();
    elements[first].y_vel += mag * theta.sin() * -dy.signum();

    elements[second].x_vel += (-mag) * theta.cos() * dx.signum();
    elements[second].y_vel += (-mag) * theta.sin() * dy.signum();
}
pub fn move_actors(actors: &mut [SimulationActor]){
    for a in actors.iter_mut(){
        a.x_pos += a.x_vel * SIMULATION_TIMESTEP;
        a.y_pos += a.y_vel * SIMULATION_TIMESTEP;
    }
}