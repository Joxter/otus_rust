use crate::Report;

pub struct SmartSocket {
    name: String,
}

impl SmartSocket {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Report for SmartSocket {
    fn get_report(&self) -> String {
        format!("Smart socket \"{}\"", self.name)
    }
}

pub struct SmartThermometer {
    name: String,
}

impl SmartThermometer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Report for SmartThermometer {
    fn get_report(&self) -> String {
        format!("Smart thermometer \"{}\"", self.name)
    }
}
