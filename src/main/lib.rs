use std::str::FromStr;
use aul::error;

use crate::config::get_env;
use crate::error::WSSError;
use crate::server::Server;

mod error;
mod io;
mod server;
mod config;

const ADDRESS: &str = "0.0.0.0";
const PORT: &str = "PORT";

const DEFAULT: u16 = 8080;

fn main() -> Result<(), WSSError> {
    config::init();
    io::init()?;
    Server::init((ADDRESS, get_port()))?.start();
    Ok(())
}

fn get_port() -> u16 {
    let env = get_env(PORT);
    if env.is_some() {
        let env = env.unwrap();
        let res = u16::from_str(env.as_str());
        if let Ok(res) = res {
            return res;
        }
        error!("Couldn't parse given PORT: {}",env);
    } else {
        error!("There was no env variable PORT provided resorting back to standard {}",DEFAULT);
    }
    DEFAULT
}