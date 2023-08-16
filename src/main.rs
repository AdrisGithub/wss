use std::fmt::Error;

use crate::net::server::Server;

pub mod net;

fn main() -> Result<(), Error> {
    Server::default().start()
}