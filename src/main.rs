/*

- Разделить логически целостные элементы библиотеки "умный дом на отдельные файлы.
- Покрыть тестами требования к библиотеке.
- Создать example использования библиотеки. Библиотека предоставляет структуру
    дома в комнатах которого расположены устройства.

 */

use crate::device_provider::DeviceInfoProvider;
use crate::smart_house::SmartHouse;

mod smart_house;
mod device_provider;

struct SmartSocket {
    name: String,
}

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
