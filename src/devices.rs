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

    fn get_status(&self) -> &str {
        match self.is_on {
            true => "ON",
            false => "OFF",
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

#[test]
fn socket_get_report_works() {
    let socket1 = SmartSocket::new("socket_1", 220);
    assert_eq!(
        "Smart socket socket_1: \"default descr\" (OFF/220W)",
        socket1.get_report()
    );
}
