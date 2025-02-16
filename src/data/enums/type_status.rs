#[derive(Debug, Clone)]
pub enum TypeStatus {
    Succes,
    Error,
    Unknown
}


impl TypeStatus {
    pub fn to_string(&self) -> String {
        match self {
            TypeStatus::Succes => "succes".to_string(),
            TypeStatus::Error => "error".to_string(),
            TypeStatus::Unknown => "unknow".to_string(),
        }
    }
}

impl From<&str> for TypeStatus {
    fn from(s: &str) -> Self {
        match s {
            "succes" =>TypeStatus::Succes,
            "error" =>TypeStatus::Error,
            _ => TypeStatus::Unknown,
        }
    }
}