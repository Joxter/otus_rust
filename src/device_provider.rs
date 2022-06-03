use std::collections::HashMap;

pub trait Report {
    fn get_report(&self) -> String;
}

pub trait DeviceInfoProvider<'a> {
    fn get_report(&self, room: &str, name: &str) -> String;
    fn add_device(&mut self, room: &str, name: &str, device: &'a dyn Report);
}

pub struct BorrowingDeviceInfoProvider<'a> {
    items: HashMap<String, &'a dyn Report>,
}

impl<'a> BorrowingDeviceInfoProvider<'a> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<'a> DeviceInfoProvider<'a> for BorrowingDeviceInfoProvider<'a> {
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
