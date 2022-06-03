use crate::DeviceInfoProvider;

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

