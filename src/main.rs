mod parser;
use parser::parser::{Interpreter};
use std::{collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}};

struct Server {
    // listener: TcpListener,
    db: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            db: HashMap::new()
        }
    }
    pub fn handle_client(&mut self, mut stream: TcpStream) -> std::io::Result<()> {
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
            interpreter.interprete(&mut self.db);
            stream.write("OK!\n".as_bytes())?;
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    let listener = TcpListener::bind("127.0.0.1:6969")?;
    for stream in listener.incoming() {
        server.handle_client(stream?)?;
    }
    Ok(())
}
