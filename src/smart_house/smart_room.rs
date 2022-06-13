use std::fmt::{Display, Formatter};

pub struct SmartRoom(Vec<String>);

#[derive(Debug)]
pub enum RoomError {
    Empty,
    TooLong,
    NotUniq,
    RoomIsFull,
}

impl Display for RoomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomError::Empty => write!(f, "Имя не может быть пустым"),
            RoomError::TooLong => write!(f, "Слишком длинное имя"),
            RoomError::NotUniq => write!(f, "Устройство с таким именем уже есть"),
            RoomError::RoomIsFull => write!(f, "В комнате нет места"),
        }
    }
}

impl From<RoomError> for String {
    fn from(err: RoomError) -> Self {
        err.to_string()
    }
}

impl SmartRoom {
    pub fn new() -> Self {
        SmartRoom(vec![])
    }

    pub fn add_device(&mut self, name: &str) -> Result<&mut Self, RoomError> {
        if self.0.len() > 30 {
            return Err(RoomError::RoomIsFull);
        }
        if name.is_empty() {
            return Err(RoomError::Empty);
        }
        if name.len() > 30 {
            return Err(RoomError::TooLong);
        }
        if self.0.iter().any(|it| it.eq(name)) {
            return Err(RoomError::NotUniq);
        }

        self.0.push(name.to_string());
        Ok(self)
    }

    pub fn get_devices(&self) -> &Vec<String> {
        &self.0
    }
}
