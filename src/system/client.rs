use std::collections::HashMap;
use std::io::{self, Error};
use get_if_addrs::get_if_addrs;
use multiplayer_fps::data::entities::clients::Client;
use multiplayer_fps::data::entities::player::Player;
use multiplayer_fps::data::entities::udp::{UDPMethod, UDP};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut ip_addr = String::new();
    let mut username = String::new();
    let interfaces = get_if_addrs().expect("Impossible de récupérer les interfaces réseau");
    for iter  in interfaces.iter() {
        println!("{:?}", iter);
    }
    let ip_client = interfaces[0].ip().to_string();
    let socket = UDP::new(8081,ip_client.as_str()).await?;
    let mut client = Client::new(username.clone(), Player::default(), socket);
    println!("Veuillez entrer votre l'addresse du server :");
    io::stdin()
        .read_line(&mut ip_addr)
        .expect("Échec de la lecture de l'entrée");
    ip_addr = ip_addr.trim().to_string();
    println!("Veuillez entrer votre id de connexion:");
    io::stdin()
        .read_line(&mut username)
        .expect("Échec de la lecture de l'entrée");
    username = username.trim().to_string();
    client.set_username(username.clone());
    let mut conn: HashMap<String, String> = HashMap::new();
    conn.insert("username".to_string(), username);
    let json_str = serde_json::to_string(&conn).expect("Erreur de sérialisation");

    client.network.send(json_str, ip_addr).await.expect("TODO: panic message");
    Ok(())
}


