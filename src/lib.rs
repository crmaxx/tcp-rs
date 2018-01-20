// Copyright © 2018, Maxim Zhukov
// Licensed under the MIT License <LICENSE.md>
use std::convert::AsRef;

mod winsock2;
pub use winsock2::{Error, Response};

#[derive(Clone, Debug)]
pub struct Client<'a, 'b> {
    host: &'a str,
    port: 'b u16,
}

impl<'a, 'b> Client<'a, 'b> {
    pub fn new<S: ?Sized + AsRef<str>>(host: &'a S, port: u16) -> Self {
        Client {
            host: host.as_ref(),
            port: port,
        }
    }

    pub fn open(self) -> Result<Response, Error> {
        Response::open(self)
    }
}
