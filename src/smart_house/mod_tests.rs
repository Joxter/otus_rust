use crate::smart_house::{DeviceInfoProvider, Report, SmartHouse};
use std::collections::HashMap;

pub struct TestDevice {
    name: String,
}

impl Report for TestDevice {
    fn get_report(&self) -> String {
        format!("report {}", self.name)
    }
}

pub struct TestProvider<'a> {
    items: HashMap<String, &'a TestDevice>,
}

impl<'a> TestProvider<'a> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn add_device(&mut self, room: &str, name: &str, device: &'a TestDevice) {
        self.items.insert(format!("{}_{}", room, name), device);
    }
}

impl<'a> DeviceInfoProvider<'a> for TestProvider<'a> {
    fn get_report(&self, room: &str, name: &str) -> String {
        match self.items.get(&format!("{}_{}", room, name)) {
            Some(device) => device.get_report(),
            None => format!("{:} is not found", name),
        }
    }
}

#[test]
fn it_works() {
    let mut house = SmartHouse::new("my house");

    house.add_room("kitchen");
    house.add_device("kitchen", "thermo_1");
    house.add_device("kitchen", "thermo_2");
    house.add_room("bedroom");

    assert_eq!(vec!["bedroom", "kitchen"], house.get_rooms());
    assert_eq!(vec!["thermo_1", "thermo_2"], house.get_devices("kitchen"));
    let empty_string_vec: Vec<String> = Vec::new();
    assert_eq!(empty_string_vec, house.get_devices("bedroom"));

    let thermo1 = TestDevice {
        name: "thermo_1".to_string(),
    };
    let mut info_provider_1 = TestProvider::new();
    info_provider_1.add_device("kitchen", "thermo_1", &thermo1);

    assert_eq!(
        "### House \"my house\"\nbedroom:\nkitchen:\n> report thermo_1\n> thermo_2 is not found\n",
        house.create_report(&info_provider_1)
    );
}
