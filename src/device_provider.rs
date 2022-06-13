use crate::smart_house::{DeviceInfoProvider, Report};
use std::collections::HashMap;

pub struct BorrowingDeviceInfoProvider<'a> {
    items: HashMap<String, &'a dyn Report>,
}

impl<'a> BorrowingDeviceInfoProvider<'a> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, room: &str, name: &str, device: &'a dyn Report) {
        self.items.insert(format!("{}_{}", room, name), device);
    }
}

impl<'a> DeviceInfoProvider<'a> for BorrowingDeviceInfoProvider<'a> {
    fn get_report(&self, room: &str, name: &str) -> Option<String> {
        self.items
            .get(&format!("{}_{}", room, name))
            .map(|device| device.get_report())
    }
}
