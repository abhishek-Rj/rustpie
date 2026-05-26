use libc;
use std::{net::TcpStream, os::fd::AsRawFd};

#[derive(Default)]
pub struct Buffer {
    data: [u8; 6],
    read_pos: usize,
    write_pos: usize
}

impl Buffer {
    pub fn read(&mut self, stream: &TcpStream) -> Result<Vec<u8>, String> {
        let fd= stream.as_raw_fd();

        let ptr = unsafe {
            self.data.as_mut_ptr().add(self.write_pos) as *mut libc::c_void
        };

        let remaining = self.data.len() - self.write_pos;
        let bytes_read = unsafe {
            libc::read(fd, ptr, remaining)
        };

        if bytes_read < 0 {
            return Err("read system call failed".into());
        } 

        self.write_pos += bytes_read as usize;
        
        Ok(self.data[self.read_pos..self.write_pos].to_vec())
    }

    pub fn write(&mut self) {

    }
}