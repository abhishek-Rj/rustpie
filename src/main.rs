mod net;
mod parser;
mod persistent_storage;

use parser::plotwhole::Rufus;
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::net::{buffer::{self, Buffer}, socket::Socket};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut rufus = parser::plotwhole::Rufus {
        data: HashMap::new(),
    };

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("plotwhole.txt")
        .unwrap();
    let mut buff_read = BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes = buff_read.read_line(&mut line).unwrap();

        if bytes == 0 {
            break;
        }
        let line = Cow::Borrowed(line.as_str());
        rufus.load_request(line);
    }

    println!("previous data loaded");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut rufus);
    }
}

fn handle_connection(stream: TcpStream, rufus: &mut Rufus) {
    let mut chunk = [0u8; 6];
    let socket = Socket::new(&stream);
    let mut buffer = Buffer::new();

    
    loop {
        let bytes_count = socket.read_into(&mut chunk).unwrap();
        if bytes_count == 0 {
            break;
        }
        
        buffer.write_bytes(&chunk[..bytes_count]);
        let msg = String::from_utf8_lossy(&buffer.data);
        println!("{msg}");
        
        if buffer.readable_slice().ends_with(b"\r\n") {
            println!("size of buffer right now: {}", buffer.data.len());
            break;
        }
    }
    
    rufus.handle_request(String::from_utf8_lossy(&buffer.data), Some(stream));
}
