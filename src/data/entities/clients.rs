use super::{player::Player, udp::UDP};

pub struct Client {
    username: String,
    player: Player,
    pub network: UDP,
}

impl Client {
    pub fn new(username: String, player: Player, network: UDP) -> Client {
        Client {
            username,
            player,
            network,
        }
    }
    pub fn ip_address(&self) -> String {
        self.network.address()
    }
    pub fn username(&self) -> String {
        self.username.clone()
    }
}
