#![allow(dead_code)]

#[derive(Default)]
pub struct Buffer {
    data: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl Buffer {
    pub fn new() -> Self {
        return Buffer {
            data: Vec::with_capacity(1024),
            ..Default::default()
        };
    }

    pub fn write_bytes(&mut self, chunk: &mut [u8]) -> () {
        self.data.extend_from_slice(chunk);
        self.write_pos += chunk.len();
    }

    pub fn readable_slice(&self) -> &[u8] {
        return &self.data[self.read_pos..self.write_pos]
    }

    pub fn consume(&mut self, n: usize) {
        self.read_pos += n;
    }

    pub fn compact(&mut self) {
        self.data.copy_within(self.read_pos..self.write_pos, 0);
        let len= self.write_pos - self.read_pos;
        self.data.truncate(len);
        self.write_pos = len;
        self.read_pos = 0;
    }
}
