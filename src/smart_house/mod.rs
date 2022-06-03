use crate::DeviceInfoProvider;
use std::collections::HashMap;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Vec<String>>,
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
        self.rooms.clone().into_keys().collect()
    }

    pub fn get_devices(&self, room: &str) -> Vec<String> {
        if let Some(r) = self.rooms.get(room) {
            return r.clone();
        }
        vec![]
    }

    pub fn create_report<'a, T: DeviceInfoProvider<'a>>(&self, provider: &T) -> String {
        let mut report = format!("### House \"{}\"\n", self.name).clone();

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
