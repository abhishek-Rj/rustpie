#![allow(dead_code)]

use libc;
use std::{net::TcpStream, os::fd::{AsRawFd, RawFd}};

pub struct Socket {
    fd: RawFd,
}

impl Socket {
    pub fn new (stream: &TcpStream) -> Self {
        return Socket {fd: stream.as_raw_fd() };
    }

    pub fn read (&self, buf: &mut [u8]) -> Result<usize, String>{
        let n = unsafe {
            libc::read(
                self.fd, 
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len()
            )
        };

        if n < 0 {
            return Err("Fuck u nga! read sys call failed".into());
        }
        return Ok(n as usize);
    }
}
