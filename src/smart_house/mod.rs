use std::collections::HashMap;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Vec<String>>,
}

pub trait Report {
    fn get_report(&self) -> String;
}

pub trait DeviceInfoProvider<'a> {
    fn get_report(&self, room: &str, name: &str) -> String;
    fn add_device(&mut self, room: &str, name: &str, device: &'a dyn Report);
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: &str) {
        self.rooms.insert(name.to_string(), vec![]);
    }

    pub fn add_device(&mut self, room: &str, device: &str) {
        if let Some(r) = self.rooms.get_mut(room) {
            r.push(device.to_string())
        };
    }

    pub fn get_rooms(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.rooms.clone().into_keys().collect();
        // сортировка только для стабильности результата
        keys.sort();
        keys
    }

    pub fn get_devices(&self, room: &str) -> Vec<String> {
        if let Some(r) = self.rooms.get(room) {
            return r.clone();
        }
        vec![]
    }

    pub fn create_report<'a, T: DeviceInfoProvider<'a>>(&self, provider: &T) -> String {
        let mut report = format!("### House \"{}\"\n", self.name);

        for room in self.get_rooms() {
            report.push_str(format!("{}:\n", room).as_str());

            for device in self.get_devices(room.as_str()) {
                let device_report = provider.get_report(room.as_str(), device.as_str());
                report.push_str(format!("> {}\n", device_report).as_str());
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_house::{DeviceInfoProvider, Report, SmartHouse};
    use std::collections::HashMap;
    // use crate::SmartHouse;

    pub struct TestDevice {
        name: String,
    }

    impl Report for TestDevice {
        fn get_report(&self) -> String {
            format!("report {}", self.name)
        }
    }

    pub struct TestProvider<'a> {
        items: HashMap<String, &'a dyn Report>,
    }

    impl<'a> TestProvider<'a> {
        pub fn new() -> Self {
            Self {
                items: HashMap::new(),
            }
        }
    }

    impl<'a> DeviceInfoProvider<'a> for TestProvider<'a> {
        fn get_report(&self, room: &str, name: &str) -> String {
            match self.items.get(&format!("{}_{}", room, name)) {
                Some(device) => device.get_report(),
                None => format!("{:} is not found", name),
            }
        }

        fn add_device(&mut self, room: &str, name: &str, device: &'a dyn Report) {
            self.items.insert(format!("{}_{}", room, name), device);
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

        assert_eq!("### House \"my house\"\nbedroom:\nkitchen:\n> report thermo_1\n> thermo_2 is not found\n", house.create_report(&info_provider_1));
    }
}
