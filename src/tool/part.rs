#[derive(Debug, Clone)]
pub struct Part {
    pub serial_no: String,
    pub part_no: String,
    steps_completed: Vec<String>,
    pub failed: bool    
}

impl Part {
    pub fn new(serial_no: &str, part_no: &str) -> Self {
        Self { 
            serial_no: serial_no.to_string(),
            part_no: part_no.to_string(),
            steps_completed: vec![],
            failed: false
        }
    }

    pub fn add_step(&mut self, step_id: &str) {
        self.steps_completed.push(step_id.to_string());
    }

    pub fn serial_no(&self) -> String {
        self.serial_no.clone()
    }


    pub fn part_no(&self) -> String {
        self.part_no.clone()
    }
}
