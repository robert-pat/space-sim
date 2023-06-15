pub struct SimulationContainer{
    is_running: bool,
}

impl SimulationContainer{
    pub fn new() -> Self{
        Self{ is_running: false}
    }
    pub fn suspend(&mut self){}
    pub fn resume(&mut self){}
    pub fn load_from_file(&mut self, path: &str){
        todo!()
        // needs to handle empty string, ""
    }
    pub fn step(&mut self){
        //TODO: calc forces & move all particles to their next position
        //TODO: add timing checks to ensure the sim updates independently from frame rate
    }
}
impl Default for SimulationContainer{
    fn default() -> Self {
        SimulationContainer::new()
    }
}
