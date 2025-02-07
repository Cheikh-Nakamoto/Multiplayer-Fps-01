pub struct UDP {
    port: u32,
    address: String,
}

trait UDPMethod {
    fn send(&self, message: String);
    fn receive(&self) -> String;
}

impl UDP {
    pub fn new(port: u32, address: String) -> UDP {
        UDP {
            port,
            address,
        }
    }
    pub fn port(&self) -> u32 {
        self.port
    }
    pub fn address(&self) -> String {
        self.address
    }
}