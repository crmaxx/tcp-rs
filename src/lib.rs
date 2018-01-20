// Copyright © 2018, Maxim Zhukov
// Licensed under the MIT License <LICENSE.md>

use std::net::{IpAddr, SocketAddr};

mod winsock2;
pub use winsock2::{Error, Socket};

#[derive(Clone, Debug)]
pub struct Client {
    addr: SocketAddr,
}

impl Client {
    pub fn new(addr: &SocketAddr) -> Self {
        Client {
            addr: addr,
        }
    }

    pub fn open(self) -> Result<Socket, Error> {
        Socket::open(self)
    }
}
