mod parser;
use parser::parser::{Interpreter};
use std::{collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}};

struct Server {
    db: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            db: HashMap::new()
        }
    }
    pub fn handle_client(&mut self, mut stream: TcpStream) -> std::io::Result<()> {
        println!("INFO: Client connected. address: <{}>", &stream.peer_addr().unwrap());
        loop {
            let mut buffer = [0u8; 256];
            stream.write("> ".as_bytes())?;
            let n = stream.read(&mut buffer)?;
            let res = str::from_utf8(&buffer).unwrap()[..n].trim();
            if res == "quit" {
                break;
            }

            let mut interpreter = Interpreter::new(&res);
            interpreter.lexer.tokenize();
            let ret = interpreter.interprete(&mut self.db);
            stream.write(format!("{ret}\n").as_bytes())?;
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    let addr = "127.0.0.1:6969";
    let listener = match TcpListener::bind(addr) {
        Ok(l) => {
            println!("INFO: The server is listening on <{addr}>");
            l
        }
        Err(err) => {
            panic!("ERROR: Failed binding address because of this error ({err})");
        }
    };

    for stream in listener.incoming() {
        server.handle_client(stream?)?;
    }
    Ok(())
}
