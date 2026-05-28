mod net;
mod parser;
mod persistent_storage;

use parser::plotwhole::Rufus;
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};

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

fn handle_connection(mut stream: TcpStream, rufus: &mut Rufus) {
    let mut buffer = [0u8; 6];
    let mut actual_data: Vec<_> = vec![];
    let mut flat: Vec<u8> = vec![];
    let mut req: Cow<'_, str> = Cow::Owned("".to_string());

    loop {
        let buff_read = stream.read(&mut buffer).unwrap();

        if buff_read == 0 {
            break;
        }

        actual_data.push(buffer[..buff_read].to_vec());
        // println!("chunks recieved {:?}", &buffer[..buff_read]);
        // println!("{}", String::from_utf8_lossy(&buffer[..buff_read]));

        flat = actual_data.iter().flatten().cloned().collect();
        req = String::from_utf8_lossy(&flat);

        if req.ends_with("\r\n") {
            println!("Full Request: {}", req);
            break;
        }
    }

    rufus.handle_request(req, Some(stream));
}
