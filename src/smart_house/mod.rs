#[cfg(test)]
mod mod_tests;

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

/// # Умный дом
///
/// ## Можно:
/// - добавлять комнаты
/// - добавлять устройства в комнаты
/// - получить все комнаты
/// - получить устройства в комнате
/// - получить отчет по всем устройствам в доме
///
/// ```
/// let mut house = SmartHouse::new("my house");
///
/// house.add_room("kitchen");
/// house.add_device("kitchen", "socket_1");
/// house.add_device("kitchen", "thermo_1");
/// house.add_room("bedroom");
/// house.add_device("bedroom", "thermo_1");
///
/// println!("get_rooms: {:?}", house.get_rooms());
/// println!("get_devices(kitchen): {:?}", house.get_devices("kitchen"));
/// println!("get_devices(bedroom): {:?}", house.get_devices("bedroom"));
///
/// let socket1 = SmartSocket::new("socket_1", 220);
/// let thermo1 = SmartThermometer::new("thermo_1", 24);
///
/// let mut info_provider_1 = BorrowingDeviceInfoProvider::new();
/// info_provider_1.add_device("kitchen", "socket_1", &socket1);
/// info_provider_1.add_device("kitchen", "thermo_1", &thermo1);
///
/// println!("report: \n{}", house.create_report(&info_provider_1));
/// ```

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Vec<String>>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct DeviceKey(String);
impl Display for DeviceKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Трейт для отчетов, его должны реализовывать все устройства в доме
pub trait Report {
    fn get_report(&self) -> String;
}

/// Трейт для получения отчета утройства по его комнате и имени.
///
/// Должен быть имплементирован пользовтаелем на хранилище устройств и передаваться в
/// метод `create_report` дома.
pub trait DeviceInfoProvider<'a> {
    fn get_report(&self, device_key: DeviceKey) -> String;
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: &str) {
        self.rooms.insert(name.to_string(), vec![]);
    }

    pub fn add_device<'a>(&mut self, room: &'a str, device: &'a str) -> DeviceKey {
        if let Some(r) = self.rooms.get_mut(room) {
            r.push(device.to_string())
        };
        Self::get_device_key(room, device)
    }

    fn get_device_key(room: &str, device: &str) -> DeviceKey {
        DeviceKey(format!("{}_{}", room, device))
    }

    pub fn get_rooms(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.rooms.clone().into_keys().collect();
        // сортировка только для стабильности результата
        keys.sort();
        keys
    }

    pub fn get_devices(&self, room: &str) -> Vec<String> {
        if let Some(r) = self.rooms.get(room) {
            return r.clone();
        }
        vec![]
    }

    pub fn create_report<'a, T: DeviceInfoProvider<'a>>(&self, provider: &T) -> String {
        let mut report = format!("### House \"{}\"\n", self.name);

        for room in self.get_rooms() {
            report.push_str(format!("{}:\n", room).as_str());

            for device in self.get_devices(room.as_str()) {
                let device_report =
                    provider.get_report(Self::get_device_key(room.as_str(), device.as_str()));
                report.push_str(format!("> {}\n", device_report).as_str());
            }
        }

        report
    }
}
