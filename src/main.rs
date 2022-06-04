use crate::device_provider::BorrowingDeviceInfoProvider;
use crate::devices::{SmartSocket, SmartThermometer};
use crate::smart_house::SmartHouse;

mod device_provider;
mod devices;
mod smart_house;

fn main() {
    let socket1 = SmartSocket::new("socket_1", 220);
    let thermo1 = SmartThermometer::new("thermo_1", 24);

    let mut house = SmartHouse::new("my house");
    let mut info_provider_1 = BorrowingDeviceInfoProvider::new();

    house.add_room("kitchen"); // todo может тут возвращать комнату? чтоб можно было так:
                               //        let mut kitchen = house.create_room("kitchen");
                               //        kitchen.mount_device("socket_1");
    info_provider_1.add_device(house.add_device("kitchen", "socket_1"), &socket1);
    info_provider_1.add_device(house.add_device("kitchen", "thermo_1"), &thermo1);
    house.add_room("bedroom");
    house.add_device("bedroom", "thermo_1");

    println!("get_rooms: {:?}", house.get_rooms());
    println!("get_devices(kitchen): {:?}", house.get_devices("kitchen"));
    println!("get_devices(bedroom): {:?}", house.get_devices("bedroom"));

    println!("report: \n{}", house.create_report(&info_provider_1));
}
