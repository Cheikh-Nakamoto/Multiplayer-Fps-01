pub mod data;

use crate::data::entities::{
    game::Game,
    server::{Server, ServerMethod},
    udp::UDP,
};
use std::io::Error;
//pub use bevy::math::Vec3;
pub mod nalgebra;
pub use nalgebra::Vec3;

pub async fn run_server() -> Result<(), Error> {
    let multicast_addr = "239.1.2.3";
    let udp = UDP::new(8080, "0.0.0.0").await?;
    let server = Server::new(vec![], Game::new(), udp);
    server.run().await;
    Ok(())
}
