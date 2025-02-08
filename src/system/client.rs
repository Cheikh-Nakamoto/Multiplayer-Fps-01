use std::collections::HashMap;
use std::io::{self, Error};
use get_if_addrs::get_if_addrs;
use multiplayer_fps::data::entities::clients::{Client, ClientMethods};
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::data::entities::udp::{UDPMethod, UDP};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let  (username, ip_addr) = Client::collect()?;
    let interfaces = get_if_addrs().expect("Impossible de récupérer les interfaces réseau");
    for iter  in interfaces.iter() {
        println!("{:?}", iter);
    }
    let ip_client = interfaces[0].ip().to_string();
    let socket = UDP::new(8081,ip_client.as_str()).await?;
    let mut client = Client::new(username.clone(), Player::default(), socket);
    client.set_username(username.clone());
    let mut conn: HashMap<String, String> = HashMap::new();
    conn.insert("username".to_string(), username);
    let json_str = serde_json::to_string(&conn).expect("Erreur de sérialisation");

    client.network.send(json_str, ip_addr).await.expect("TODO: panic message");
    Ok(())
}


