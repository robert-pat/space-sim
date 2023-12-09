const GRAVITATIONAL_CONSTANT: f64 = 1.0;
const SIMULATION_TIME_STEP: f64 = 0.05;
const SIMULATION_PRUNE_ZONE: f64 = 100000f64; // Hundred thousand
const NEGLIGIBLE_RADIUS: u32 = 10;

#[derive(Clone)]
#[allow(unused)]
pub struct SimulationActor {
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    mass: f64,
    radius: u32, // u32 bc (rn) only used for rendering; maybe will or should change
    color: [u8; 4],
}
impl SimulationActor {
    pub fn new(x: f64, y: f64, mass: f64, color: [u8; 4]) -> Self {
        Self {
            x_pos: x,
            y_pos: y,
            x_vel: 0.0,
            y_vel: 0.0,
            mass,
            radius: 100,
            color,
        }
    }
    pub fn coordinates(&self) -> (f64, f64) {
        (self.x_pos, self.y_pos)
    }
    #[allow(unused)]
    pub fn change_color(&mut self, color: [u8; 4]) {
        self.color = color
    }
    pub fn get_color(&self) -> [u8; 4] {
        self.color
    }
    pub fn radius(&self) -> u32 {
        self.radius
    }
}
impl Default for SimulationActor {
    fn default() -> Self {
        SimulationActor::new(0.0, 0.0, 1.0, [255u8; 4])
    }
}

pub struct SimulationContainer {
    pub(crate) is_running: bool,
    pub(crate) space: Vec<SimulationActor>,
    pub(crate) prev_step: std::time::SystemTime,
}
impl SimulationContainer {
    pub fn new() -> Self {
        Self {
            is_running: false,
            space: vec![],
            prev_step: std::time::SystemTime::now(),
        }
    }
    pub fn suspend(&mut self) {
        self.is_running = false;
    }
    pub fn resume(&mut self) {
        self.is_running = true;
    }
    pub fn step(&mut self) {
        self.prev_step = std::time::SystemTime::now();
        for inner in 0..self.space.len() {
            for outer in inner..self.space.len() {
                if inner == outer {
                    continue;
                }
                apply_gravity(&mut self.space, inner, outer);
            }
        }
        move_actors(&mut self.space);
    }
    pub fn add_actor(&mut self, a: SimulationActor) {
        self.space.push(a);
    }
    #[allow(unused)]
    pub fn prune(&mut self) {
        self.space.retain(|actor| {
            actor.x_pos >= SIMULATION_PRUNE_ZONE
                || actor.y_pos >= SIMULATION_PRUNE_ZONE
                || actor.radius <= NEGLIGIBLE_RADIUS
        });
    }
}
impl Default for SimulationContainer {
    fn default() -> Self {
        SimulationContainer::new()
    }
}

pub fn apply_gravity(elements: &mut [SimulationActor], first: usize, second: usize) {
    let (f, s) = (&elements[first], &elements[second]);
    let (dx, dy) = (f.x_pos - s.x_pos, f.y_pos - s.y_pos);

    let r = (dx.powf(2f64) + dy.powf(2f64)).sqrt();
    let mag = GRAVITATIONAL_CONSTANT * f.mass * s.mass * r.recip().powf(2f64);
    let theta = (dy / dx).atan();

    elements[first].x_vel += mag * theta.cos() * -dx.signum();
    elements[first].y_vel += mag * theta.sin() * -dy.signum();

    elements[second].x_vel += mag * theta.cos() * dx.signum();
    elements[second].y_vel += mag * theta.sin() * dy.signum();
}
pub fn move_actors(actors: &mut [SimulationActor]) {
    for a in actors.iter_mut() {
        a.x_pos += a.x_vel * SIMULATION_TIME_STEP;
        a.y_pos += a.y_vel * SIMULATION_TIME_STEP;
    }
}
