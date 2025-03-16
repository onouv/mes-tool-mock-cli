#[derive(Debug)]
pub struct Part {
    serial_no: String,
    part_no: String
}

impl Part {
    pub fn new(serial_no: &str, part_no: &str) -> Self {
        Self { 
            serial_no: serial_no.to_string(),
            part_no: part_no.to_string()
        }
    }
}
