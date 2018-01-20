// Copyright © 2018, Maxim Zhukov
// Licensed under the MIT License <LICENSE.md>
use std::convert::AsRef;

mod winsock2;
pub use winsock2::{Error, Response};

#[derive(Clone, Debug)]
pub struct Client<'a> {
    host: &'a str,
    port: u16,
}

impl<'a> Client<'a> {
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
