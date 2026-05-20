use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};

pub struct Rufus {
    pub data: HashMap<String, String>
}

impl Rufus {
    pub fn plotwhole (&mut self, req: Cow<'_, str>, mut stream: TcpStream) {
        let req = req.into_owned();
        let req = req.trim();
        let req_iterator: Vec<String> = req.split(' ').map(|s| s.to_string()).collect();
        
        println!("{:?}", req_iterator);
        if req_iterator[0] == "SET" {
            if req_iterator.len() != 3 {
                return;
            }
            self.data.insert(req_iterator[1].clone(), req_iterator[2].clone());
            if let Ok(_) = stream.write_all(b"value inserted\n") {
                println!("Success");
            } else {
                println!("Err");
            }
        }

        if req_iterator[0] == "GET" {
            if req_iterator.len() != 2 {
                return;
            }
            let key = req_iterator[1].clone();
            if let Some(value) = self.data.get(&key) {
                if let Ok(x) = stream.write_all(value.as_bytes()) {
                    stream.write_all(b"\n").unwrap();
                    println!("Success")
                } else {
                    println!("Err")
                }
            } else {
                println!("Coudn't find the data");
            }
        }

        if req_iterator[0] == "DELETE" {
            if req_iterator.len() != 2 {
                return;
            }
            let key = req_iterator[1].clone();
            if let Some(val) = self.data.remove(&key) {
                if let Ok(x) = stream.write_all(val.as_bytes()) {
                    stream.write_all(b"\n").unwrap();
                }
                println!("{}", val);
            } else {
                println!("didn't delete from db");
            }
        }

        if req_iterator[0] == "FLUSHALL" {
            if req_iterator.len() != 1 {
                return;
            }
            self.data.clear();           
            if let Ok(_) = stream.write_all(b"deleted all records") {
                stream.write_all(b"\n").unwrap();
                println!("Sucess");
                println!("{:?}", self.data);
            }
        }

        if req_iterator[0] == "SHOWALL" {
            if req_iterator.len() != 1 {
                return;
            }
            if let Ok(_) = stream.write_all(b"all data") {
                stream.write_all(b"\n").unwrap();
                println!("{:?}", self.data);
            }
        }
    }
}