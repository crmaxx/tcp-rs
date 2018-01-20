// Copyright © 2018, Maxim Zhukov
// Licensed under the MIT License <LICENSE.md>
extern crate winapi;

use std::{io, mem, ptr};

use winapi::shared::ntdef::NULL;
use winapi::shared::ws2def::{SOCKADDR_IN, AF_INET, SOCK_STREAM};
use winapi::um::winsock2::{gethostbyname, hostent, htons, WSAGetLastError, WSAESHUTDOWN, INVALID_SOCKET, 
                           socket, closesocket, recv, WSAStartup, WSACleanup, SOCKET};

pub type Error = io::Error;

#[derive(Debug)]
pub struct Response {
    socket: SOCKET,
}

impl Response {
    pub fn open(client: ::Client) -> Result<Response, Error> {
        let mut wsaData: WSADATA = unsafe { mem::zeroed() };
        wsa_startup(&mut wsaData).unwrap();
        
        let hostName = get_host_by_name(client.host).unwrap();
        let server = SOCKADDR_IN {
            sin_addr.S_un.S_addr = hostName->h_addr_list[0];
            sin_family = AF_INET;
            sin_port = ws2_htons(client.port).unwrap();
        };
        let mut socket: SOCKET = ptr::null();
    }
}

impl Drop for Response {
    fn drop(&mut self) {
        close_socket(self.socket).unwrap();
        wsa_cleanup().unwrap();
    }
}

impl io::Read for Response {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ws2_recv(self.socket, &mut buf)
    }
}

fn close_socket(socket: SOCKET) -> io::Result<()> {
    unsafe {
        match closesocket(socket) {
            0 => Ok(()),
            _ => Err(last_error()),
        }
    }
}

fn wsa_startup(&mut wsaData: WSADATA) -> io::Result<()> {
    unsafe {
        match WSAStartup(0x202, &mut wsaData) {
            0 => Ok(()),
            _ => Err(last_error()),
        }
    }
}

fn wsa_cleanup() -> io::Result<()> {
    unsafe {
        match WSACleanup() {
            0 => Ok(()),
            _ => Err(last_error()),
        }
    }
}

fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { WSAGetLastError() })
}

fn get_host_by_name(host: &str) -> io::Result<hostent> {
    unsafe {
        match gethostbyname(host) {
            ptr if ptr.is_null() => Err(last_error()),
            ptr => Ok(ptr),
        }
    }
}

fn ws2_htons(hostshort: u16) -> io::Result<u16> {
    unsafe {
        match htons(hostshort) {
            n => Ok(n as usize),
            _ => Err(last_error()),
        }
    }
}

fn ws2_socket() -> io::Result<SOCKET> {
    unsafe {
        match socket(AF_INET, SOCK_STREAM, NULL) {
            INVALID_SOCKET => Err(last_error()),
            sckt => Ok(sckt as SOCKET)
        }
    }
}

fn ws2_recv(socket: SOCKET, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        match recv(socket, &mut buf, buf.len(), 0) {
            -1 if WSAGetLastError() == WSAESHUTDOWN => Ok(0),
            -1 => Err(last_error()),
            n => Ok(n as usize)
        }
    }
}
