pub trait DeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> String;
}

