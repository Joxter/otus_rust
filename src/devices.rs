use crate::smart_house::Report;

pub struct SmartSocket {
    name: String,
    is_on: bool,
    descr: String,
    power: u32,
}

impl SmartSocket {
    pub fn new(name: &str, power: u32) -> Self {
        Self {
            name: name.to_string(),
            is_on: false,
            descr: "default descr".to_string(),
            power,
        }
    }
}

impl Report for SmartSocket {
    fn get_report(&self) -> String {
        let status = if self.is_on { "ON" } else { "OFF" };
        format!(
            "Smart socket {}: \"{}\" ({}/{}W)",
            self.name, self.descr, status, self.power
        )
    }
}

pub struct SmartThermometer {
    name: String,
    t: i32,
}

impl SmartThermometer {
    pub fn new(name: &str, t: i32) -> Self {
        Self {
            name: name.to_string(),
            t,
        }
    }
}

impl Report for SmartThermometer {
    fn get_report(&self) -> String {
        format!("Smart thermometer \"{}\" ({}°)", self.name, self.t)
    }
}
