use super::udp::UDP;
pub struct Client {
    ip_address: String,
    username: String,
    network: UDP,
}

trait ClientMethod {
    fn connect(&self, ip_address: String) -> bool;
    fn send_message(&self, message: String);
    fn receive_message(&self) -> String;
}

impl Client {
    pub fn new(ip_address: String, username: String, network: UDP) -> Client {
        Client {
            ip_address,
            username,
            network,
        }
    }
    pub fn ip_address(&self) -> String {
        self.ip_address
    }
    pub fn username(&self) -> String {
        self.username
    }
    pub fn network(&self) -> UDP {
        self.network
    }
}