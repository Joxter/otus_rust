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

    pub fn add_device(&mut self, device_key: String, device: &'a dyn Report) {
        self.items.insert(device_key, device);
    }
}

impl<'a> DeviceInfoProvider<'a> for BorrowingDeviceInfoProvider<'a> {
    fn get_report(&self, device_key: String) -> String {
        match self.items.get(&device_key) {
            Some(device) => device.get_report(),
            None => format!("{:} is not found", device_key),
        }
    }
}
