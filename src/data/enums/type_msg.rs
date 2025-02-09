#[derive(Debug, Clone)]
pub enum TypeMessage {
    Movement,
    Connection,
    Disconnection,
    Unknown,
}


impl TypeMessage {
    pub fn to_string(&self) -> String {
        match self {
            TypeMessage::Movement => "movement".to_string(),
            TypeMessage::Connection => "connection".to_string(),
            TypeMessage::Disconnection => "disconnection".to_string(),
            TypeMessage::Unknown => "unknown".to_string(),
        }
    }
}

impl From<&str> for TypeMessage {
    fn from(s: &str) -> Self {
        match s {
            "movement" => TypeMessage::Movement,
            "connection" => TypeMessage::Connection,
            "disconnection" => TypeMessage::Disconnection,
            _ => TypeMessage::Unknown,
        }
    }
}