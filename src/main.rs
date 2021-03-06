use crate::device_provider::BorrowingDeviceInfoProvider;
use crate::devices::{SmartSocket, SmartThermometer};
use crate::smart_house::SmartHouse;

mod device_provider;
mod devices;
mod smart_house;

fn init_my_house() -> Result<SmartHouse, String> {
    let mut house = SmartHouse::new("my house");

    house.add_room("kitchen")?;
    if let Some(room) = house.get_mut_room("kitchen") {
        room.add_device("socket_1")?.add_device("thermo_1")?;
    }

    house.add_room("bedroom")?;
    if let Some(room) = house.get_mut_room("bedroom") {
        room.add_device("thermo_1")?;
    }

    Ok(house)
}

fn main() -> Result<(), String> {
    let house = init_my_house()?;

    println!("get_rooms: {:?}", house.get_room_names());
    println!(
        "kitchen devices: {:?}",
        house.get_room("kitchen").unwrap().get_devices()
    );
    println!(
        "bedroom devices: {:?}",
        house.get_room("bedroom").unwrap().get_devices()
    );

    let socket1 = SmartSocket::new("socket_1", 220);
    let thermo1 = SmartThermometer::new("thermo_1", 24);

    let mut info_provider_1 = BorrowingDeviceInfoProvider::new();
    info_provider_1.add_device("kitchen", "socket_1", &socket1);
    info_provider_1.add_device("kitchen", "thermo_1", &thermo1);

    println!("report: \n{}", house.create_report(&info_provider_1));

    Ok(())
}
