#[cfg(test)]
mod mod_tests;

use std::collections::HashMap;

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

pub struct SmartRoom(Vec<String>);

impl SmartRoom {
    pub fn add_device(&mut self, device: &str) -> &mut Self {
        self.0.push(device.to_string());
        self
    }

    pub fn get_devices(&self) -> &Vec<String> {
        &self.0
    }
}

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, SmartRoom>,
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
    fn get_report(&self, room: &str, name: &str) -> String;
}

impl SmartHouse {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: &str) {
        self.rooms.insert(name.to_string(), SmartRoom(vec![]));
    }

    pub fn get_room(&mut self, name: &str) -> Option<&SmartRoom> {
        self.rooms.get(name)
    }

    pub fn get_mut_room(&mut self, name: &str) -> Option<&mut SmartRoom> {
        self.rooms.get_mut(name)
    }

    pub fn get_room_names(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.rooms.keys().into_iter().cloned().collect();
        // сортировка только для стабильности результата
        keys.sort();
        keys
    }

    pub fn create_report<'a, T: DeviceInfoProvider<'a>>(&self, provider: &T) -> String {
        let mut report = format!("### House \"{}\"\n", self.name);

        for (room_name, smart_room) in self.rooms.iter() {
            report.push_str(format!("{}:\n", room_name).as_str());

            for device_name in smart_room.get_devices() {
                let device_report = provider.get_report(room_name, device_name);
                report.push_str(format!("> {}\n", device_report).as_str());
            }
        }

        report
    }
}
