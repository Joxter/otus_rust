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

    #[allow(dead_code)]
    fn turn_on(&mut self) {
        self.is_on = true
    }

    #[allow(dead_code)]
    fn turn_off(&mut self) {
        self.is_on = false
    }

    fn get_descr(&self) -> String {
        self.descr.clone()
    }

    fn get_status(&self) -> String {
        if self.is_on {
            "ON".to_string()
        } else {
            "OFF".to_string()
        }
    }

    fn get_power(&self) -> u32 {
        self.power
    }
}

impl Report for SmartSocket {
    fn get_report(&self) -> String {
        format!(
            "Smart socket {}: \"{}\" ({}/{}W)",
            self.name,
            self.get_descr(),
            self.get_status(),
            self.get_power()
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
