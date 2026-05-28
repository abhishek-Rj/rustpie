#![allow(dead_code)]

#[derive(Default)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub read_pos: usize,
    pub write_pos: usize,
}

impl Buffer {
    pub fn new() -> Self {
        return Buffer {
            data: Vec::with_capacity(512),
            ..Default::default()
        };
    }

    pub fn write_bytes(&mut self, chunk: &[u8]) -> () {
        self.data.extend_from_slice(chunk);
        self.write_pos += chunk.len();
    }

    pub fn readable_slice(&self) -> &[u8] {
        return &self.data[self.read_pos..self.write_pos];
    }

    //NO actual use just for learning a compact buffer
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
