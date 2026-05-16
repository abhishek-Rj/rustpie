mod buffer;

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

#[allow(unused)]
struct Buffer {
    data: [u8; 6],
    start: usize,
    end: usize,
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = Buffer {data: [0u8; 6], start: 0, end: 0};
    
    loop {
        let buff_read = stream.read(&mut buffer).unwrap();
        if buff_read == 0 {
            return
        }

        println!(
            "Read {} bytes: {:?}",
            buff_read,
            &buffer[..buff_read]
        );

        println!(
            "As string: {}",
            String::from_utf8_lossy(&buffer[..buff_read])
        );
    }
}