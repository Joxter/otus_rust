use crate::smart_house::{DeviceInfoProvider, DeviceKey, Report};
use std::collections::HashMap;

pub struct BorrowingDeviceInfoProvider<'a> {
    items: HashMap<DeviceKey, &'a dyn Report>,
}

impl<'a> BorrowingDeviceInfoProvider<'a> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device_key: DeviceKey, device: &'a dyn Report) {
        self.items.insert(device_key, device);
    }
}

impl<'a> DeviceInfoProvider<'a> for BorrowingDeviceInfoProvider<'a> {
    fn get_report(&self, device_key: DeviceKey) -> String {
        match self.items.get(&device_key) {
            Some(device) => device.get_report(),
            None => format!("{} is not found", device_key),
        }
    }
}
