use std::io:: Error;
use get_if_addrs::get_if_addrs;
use multiplayer_fps::data::entities::clients::{Client, ClientMethods};
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::data::entities::udp:: UDP;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (username, ip_addr) = Client::collect()?;
    let interfaces = get_if_addrs().expect("Impossible de récupérer les interfaces réseau");
    for iter  in interfaces.iter() {
        println!("{:?}", iter);
    }
    let ip_client = interfaces[0].ip().to_string();
    let socket = UDP::new(8081,ip_client.as_str()).await?;
    let mut client = Client::new(username.clone(), Player::default(),String::new(), socket);
    let _ = client.connect(username,ip_addr);
    Ok(())
}


