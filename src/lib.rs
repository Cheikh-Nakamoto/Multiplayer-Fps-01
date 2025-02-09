pub mod data;
pub mod system;

use crate::data::entities::{
    game::Game,
    server::{Server, ServerMethod},
    udp::UDP,
};
use std::io::Error;

pub mod nalgebra;
pub use nalgebra::Vec3;
use crate::data::entities::udp::UDPMethod;

pub async fn run_server() -> Result<(), Error> {
    let socket = UDP::create_socket_sender(8080).await?;
    let mut server = Server::new(vec![],vec![], Game::new(), socket);
    Server::run(&mut server).await;
    Ok(())
}