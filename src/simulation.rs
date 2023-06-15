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
}

impl Default for SimulationContainer{
    fn default() -> Self {
        SimulationContainer::new()
    }
}
