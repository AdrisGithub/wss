use crate::error::WSSError;
use crate::server::Server;

mod error;
mod io;
mod server;
const ADDRESS: &str = "0.0.0.0:8080";
fn main() -> Result<(), WSSError> {
    io::init()?;
    Server::init(ADDRESS)?.start();
    Ok(())
}
