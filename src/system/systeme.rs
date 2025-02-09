use std::io:: Error;
use get_if_addrs::get_if_addrs;
use multiplayer_fps::data::entities::clients::{Client, ClientMethods};
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::data::entities::udp:: UDP;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (username, ip_addr) = Client::collect()?;
    let mut client = Client::new(username.clone(), Player::default(),String::new());
    let e = client.connect(username,ip_addr).await;
    println!("{:?}",e);
    Ok(())
}


