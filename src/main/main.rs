use crate::error::WSSError;
use crate::server::Server;

mod error;
mod io;
mod server;
mod config;

const ADDRESS: &str = "0.0.0.0:8080";
fn main() -> Result<(), WSSError> {
    config::init();
    io::init()?;
    Server::init(ADDRESS)?.start();
    Ok(())
}
