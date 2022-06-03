/*

- Разделить логически целостные элементы библиотеки "умный дом на отдельные файлы.
- Покрыть тестами требования к библиотеке.
- Создать example использования библиотеки. Библиотека предоставляет структуру
    дома в комнатах которого расположены устройства.

 */

use crate::device_provider::{DeviceInfoProvider, Report};
use crate::smart_house::SmartHouse;
use std::collections::HashMap;

mod device_provider;
mod smart_house;

struct SmartSocket {
    name: String,
}

impl Report for SmartSocket {
    fn get_report(&self) -> String {
        format!("Smart socket \"{}\"", self.name).to_string()
    }
}

struct SmartThermometer {
    name: String,
}

impl Report for SmartThermometer {
    fn get_report(&self) -> String {
        format!("Smart thermometer \"{}\"", self.name).to_string()
    }
}

struct OwningDeviceInfoProvider<'a> {
    items: HashMap<String, &'a dyn Report>,
}

impl<'a> DeviceInfoProvider<'a> for OwningDeviceInfoProvider<'a> {
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

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

/*
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
*/

fn main() {
    let mut house = SmartHouse::new("my house");

    house.add_room("kitchen");
    house.add_device("kitchen", "socket_1");
    house.add_device("kitchen", "thermo_1");
    house.add_room("bedroom");
    house.add_device("bedroom", "thermo_1");

    println!("get_rooms: {:?}", house.get_rooms());
    println!("get_devices(kitchen): {:?}", house.get_devices("kitchen"));
    println!("get_devices(bedroom): {:?}", house.get_devices("bedroom"));

    let socket1 = SmartSocket {
        name: "socket_1".to_string(),
    };
    let thermo1 = SmartThermometer {
        name: "thermo_1".to_string(),
    };
    let mut info_provider_1 = OwningDeviceInfoProvider {
        items: HashMap::new(),
    };
    info_provider_1.add_device("kitchen", "socket_1", &socket1);
    info_provider_1.add_device("kitchen", "thermo_1", &thermo1);

    println!("report: \n{}", house.create_report(&info_provider_1));

    /*

    house.create_room("bedroom");
    house.add_device("kitchen", "socket_1");
    house.add_device("kitchen", "socket_2");
    house.add_device("kitchen", "thermo_1");

    let provider_1 = OwningDeviceInfoProvider::new();
    provider_1.add_device("kitchen", "socket_1", socket1)
    provider_1.add_device("kitchen", "thermo_1", thermo1)

    let report1 = house.create_report(&info_provider_1);
    println!("Report #1: {report1}");

    */

    // let house = SmartHouse {
    //     name: "my house".to_string(),
    //     kitchen: ["socket_1", "thermo_1", "thermo_2"],
    //     bedroom: ["socket_2", "socket_3", "thermo_3"],
    // };
    //
    // let socket1 = SmartSocket {
    //     name: "socket_1".to_string(),
    // };
    // let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    //
    // let report1 = house.create_report(&info_provider_1);
    // println!("Report #1: {report1}");
    //
    // let socket2 = SmartSocket {
    //     name: "socket_2".to_string(),
    // };
    // let thermo3 = SmartThermometer {
    //     name: "thermo_3".to_string(),
    // };
    // let info_provider_2 = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermo: &thermo3,
    // };
    //
    // let report2 = house.create_report(&info_provider_2);
    // println!("\nReport #2: {report2}");
}
