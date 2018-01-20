// Copyright © 2018, Maxim Zhukov
// Licensed under the MIT License <LICENSE.md>

extern crate winapi;

use std::{self, io, mem, ptr};
use std::net::SocketAddr;
use std::ffi::CString;

use winsock2::winapi::shared::ws2def::{AF_INET, SOCK_STREAM, SOCKADDR};
use winsock2::winapi::um::winsock2::{closesocket, connect, recv, MSG_PEEK,
                                     socket, WSACleanup, WSAGetLastError, WSAStartup,
                                     INVALID_SOCKET, SOCKET, WSADATA, WSAESHUTDOWN};
use winsock2::winapi::um::ws2tcpip::socklen_t;

pub type Error = io::Error;

#[derive(Debug)]
pub struct Socket {
    socket: SOCKET,
}

impl Socket {
    pub fn open(client: ::Client) -> Result<Socket, Error> {
        wsa_startup().unwrap();
        
        let socket: SOCKET = ws2_socket().unwrap();
        ws2_connect(socket, &client.addr).unwrap();

        Ok(Socket { socket: socket })
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        ws2_close_socket(self.socket).unwrap();
        wsa_cleanup().unwrap();
    }
}

impl io::Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ws2_recv(self.socket, buf)
    }

    fn peek(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        ws2_peek(self.socket, buf)
    }
}

fn wsa_startup() -> io::Result<()> {
    unsafe {
        let mut wsaData: WSADATA = unsafe { mem::zeroed() };
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

fn ws2_socket() -> io::Result<SOCKET> {
    unsafe {
        match socket(AF_INET, SOCK_STREAM, 0) {
            INVALID_SOCKET => Err(last_error()),
            sckt => Ok(sckt as SOCKET),
        }
    }
}

fn ws2_close_socket(socket: SOCKET) -> io::Result<()> {
    unsafe {
        match closesocket(socket) {
            0 => Ok(()),
            _ => Err(last_error()),
        }
    }
}

fn ws2_connect(socket: SOCKET, addr: &SocketAddr) -> io::Result<()> {
    unsafe {
        let (addrp, len) = addr2raw(addr);
        match connect(socket, addrp, len) {
            0 => Ok(()),
            _ => Err(last_error()),
        }
    }
}

fn ws2_recv(socket: SOCKET, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        let buf_ = buf.as_ptr() as *mut _;
        match recv(socket, buf_, buf.len() as i32, 0) {
            -1 if WSAGetLastError() == WSAESHUTDOWN => Ok(0),
            -1 => Err(last_error()),
            n => Ok(n as usize),
        }
    }
}

fn ws2_peek(socket: SOCKET, buf: &mut [u8]) -> io::Result<usize> {
    unsafe {
        let buf_ = buf.as_ptr() as *mut _;
        match recv(socket, buf_, buf.len() as i32, MSG_PEEK) {
            -1 if WSAGetLastError() == WSAESHUTDOWN => Ok(0),
            -1 => Err(last_error()),
            n => Ok(n as usize),
        }
    }
}

fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { WSAGetLastError() })
}

fn addr2raw(addr: &SocketAddr) -> (*const SOCKADDR, socklen_t) {
    match *addr {
        SocketAddr::V4(ref a) => (
            a as *const _ as *const _,
            mem::size_of_val(a) as socklen_t,
        ),
        SocketAddr::V6(ref a) => (
            a as *const _ as *const _,
            mem::size_of_val(a) as socklen_t,
        ),
    }
}
