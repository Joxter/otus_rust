pub trait Report {
    fn get_report(&self) -> String;
}

pub trait DeviceInfoProvider<'a> {
    fn get_report(&self, room: &str, name: &str) -> String;
    fn add_device(&mut self, room: &str, name: &str, device: &'a dyn Report);
}
