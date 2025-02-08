pub mod data;

use crate::data::entities::{
    game::Game,
    server::{Server, ServerMethod},
    udp::UDP,
};
use std::io::Error;
use get_if_addrs::get_if_addrs;

//pub use bevy::math::Vec3;
pub mod nalgebra;
pub use nalgebra::Vec3;

pub async fn run_server() -> Result<(), Error> {
    let interfaces = get_if_addrs().expect("Impossible de récupérer les interfaces réseau");
    for iter  in interfaces.iter() {
        println!("{:?}", iter);
    }
    let ip_client = interfaces[1].ip().to_string();
    let udp = UDP::new(8080, ip_client.as_str()).await.unwrap();
    let server = Server::new(vec![], Game::new(), udp);
    server.run().await;
    Ok(())
}
