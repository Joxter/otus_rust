struct SmartHouse<'a, 'b> {
    name: String,
    bedroom: [&'a str; 3],
    kitchen: [&'b str; 3],
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

    fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
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

trait DeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> String;
}

struct SmartSocket {
    name: String,
}

#[allow(dead_code)]
struct SmartThermometer {
    name: String,
}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> String {
        if room.eq("kitchen") && device.eq(self.socket.name.as_str()) {
            format!("Report for {:} (wip)", self.socket.name)
        } else {
            format!("{:} is not found", device)
        }
    }
}

#[allow(dead_code)]
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_info(&self, room: &str, device: &str) -> String {
        if room.eq("bedroom") {
            if device.eq(self.socket.name.as_str()) {
                format!("Report for {:} (wip)", self.socket.name)
            } else if device.eq(self.thermo.name.as_str()) {
                format!("Report for {:} (wip)", self.thermo.name)
            } else {
                format!("{:} is not found", device)
            }
        } else {
            format!("{:} is not found", device)
        }
    }
}

fn main() {
    // Инициализация дома
    let house = SmartHouse {
        name: "my house".to_string(),

        kitchen: ["socket_1", "thermo_1", "thermo_2"],
        bedroom: ["socket_2", "socket_3", "thermo_3"],
    };

    let socket1 = SmartSocket {
        name: "socket_1".to_string(),
    };
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

    let report1 = house.create_report(&info_provider_1);
    println!("Report #1: {report1}");

    let socket2 = SmartSocket {
        name: "socket_2".to_string(),
    };
    let thermo3 = SmartThermometer {
        name: "thermo_3".to_string(),
    };
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo3,
    };

    let report2 = house.create_report(&info_provider_2);
    println!("\nReport #2: {report2}");
}
