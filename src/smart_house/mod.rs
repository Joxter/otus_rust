use crate::DeviceInfoProvider;
use std::collections::HashMap;

pub struct SmartHouse<'a, 'b> {
    pub name: String,
    pub bedroom: [&'a str; 3],
    pub kitchen: [&'b str; 3],
}

impl SmartHouse<'static, 'static> {
    #[allow(dead_code)]
    fn get_rooms(&self) -> [&str; 2] {
        ["bedroom", "kitchen"]
    }

    #[allow(dead_code)]
    fn devices(&self, room: &str) -> [&str; 3] {
        match room {
            "kitchen" => self.kitchen,
            "bedroom" => self.bedroom,
            _ => ["", "", ""],
        }
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = self.name.clone();

        report.push_str("\nBedroom: \n");
        report.push_str(
            self.bedroom
                .map(|device| provider.get_info("bedroom", device))
                .join("\n")
                .as_str(),
        );

        report.push_str("\nKitchen: \n");
        report.push_str(
            self.kitchen
                .map(|device| provider.get_info("kitchen", device))
                .join("\n")
                .as_str(),
        );
        report
    }
}

pub struct SmartHouse2 {
    name: String,
    rooms: HashMap<String, Vec<String>>,
}

impl SmartHouse2 {
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

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        let mut report = format!("### House \"{}\"\n", self.name).clone();

        for room in self.get_rooms() {
            report.push_str(format!("{}:\n", room).as_str());

            for device in self.get_devices(room.as_str()) {
                let device_report = provider.get_info(room.as_str(), device.as_str());
                report.push_str(format!("> {}\n", device_report).as_str());
            }
        }

        report
    }
}
