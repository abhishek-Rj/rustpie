use crate::{persistent_storage::add_data_toa_file};
use std::fmt;
use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};

pub struct Rufus {
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

pub enum Command {
    Set(String, String, Value),
    Get(String),
    Delete(String),
    FlushAll,
    ShowAll,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(fl) => write!(f, "{fl}"),
        }
    }
}

pub fn provide_value(req: &str, req_value: &str) -> Value {
    if req == "String" {
        Value::String(req_value.to_string())
    } else if req == "Integer" {
        if let Ok(value) = req_value.parse::<i64>() {
            Value::Integer(value)
        } else {
            Value::String(req_value.to_string())
        }
    } else if req == "Float" {
        if let Ok(value) = req_value.parse::<f64>() {
            Value::Float(value)
        } else {
            Value::String(req_value.to_string())
        }
    } else {
        if let Ok(value) = req_value.parse::<bool>() {
            Value::Boolean(value)
        } else {
            Value::String(req_value.to_string())
        }
    }
} 

pub fn parse_command(req: &str) -> Result<Command, String> {
    let req = req.trim();
    let parts: Vec<String> = req.split('|').map(|s| s.to_string()).collect();

    if parts.is_empty() {
        return Err("Not a valid request".into());
    }

    match parts[0].as_str() {
        "SET" => {
            if parts.len() != 4 {
                return Err("Not a valid set request".into());
            }
            let data_type = parts[2].as_str();
            let req_value = parts[3].as_str();
            let value = provide_value(data_type, req_value);
            Ok(Command::Set(parts[1].to_string(), data_type.to_string(), value))
        }

        "GET" => {
            if parts.len() != 2 {
                return Err("Not a valid get request".into());
            }
            Ok(Command::Get(parts[1].to_string()))
        }

        "DELETE" => {
            if parts.len() != 2 {
                return Err("Not a valid delete request".into());
            }
            Ok(Command::Delete(parts[1].to_string()))
        }

        "FLUSHALL" => {
            if parts.len() != 1 {
                return Err("Not a valid FlushAll request".into());
            }
            Ok(Command::FlushAll)
        }

        "SHOWALL" => {
            if parts.len() != 1 {
                return Err("Not a valid ShowAll request".into());
            }
            Ok(Command::ShowAll)
        }

        _ => Err("Not a valie req format".into()),
    }
}

impl Rufus {
    pub fn handle_request(&mut self, req: Cow<'_, str>, mut stream: Option<TcpStream>) {
        let req = req.into_owned();

        match parse_command(&req) {
            Ok(command) => self.execute(command, stream, true),
            Err(err) => {
                if let Some(ref mut stream) = stream {
                    let _ = writeln!(stream, "Error {}", err);
                }
            }
        }
    }

    pub fn load_request(&mut self, req: Cow<'_, str>) {
        let req = req.into_owned();

        if let Ok(command) = parse_command(&req) {
            self.execute(command, None, false);
        }
    }

    fn execute(&mut self, command: Command, mut stream: Option<TcpStream>, should_persist: bool) {
        match command {
            Command::Set(key, data_type, value) => {
                let key = Cow::Borrowed(&key);
                self.data.insert(key.to_string(), value.clone());

                if should_persist {
                    let mut command = String::from("SET");
                    command.push('|');
                    command.push_str(&key.as_str());
                    command.push('|');
                    command.push_str(data_type.as_str());
                    command.push('|');
                    command.push_str(&value.to_string());
                    add_data_toa_file(command);
                }

                if let Some(ref mut stream) = stream {
                    let _ = writeln!(stream, "Success");
                }
            }

            Command::Get(key) => match self.data.get(&key) {
                Some(value) => {
                    if let Some(ref mut stream) = stream {
                        let _ = writeln!(stream, "{value}");
                    }
                }
                None => {
                    if let Some(ref mut stream) = stream {
                        let _ = writeln!(stream, "Key not found");
                    }
                }
            },

            Command::Delete(key) => match self.data.remove(&key) {
                Some(_) => {
                    if should_persist {
                        add_data_toa_file(format!("DELETE {key}"));
                    }

                    if let Some(ref mut stream) = stream {
                        let _ = writeln!(stream, "Delete {key}");
                    }
                }
                None => {
                    if let Some(ref mut stream) = stream {
                        let _ = writeln!(stream, "Key not found");
                    }
                }
            },

            Command::FlushAll => {
                self.data.clear();
                if should_persist {
                    add_data_toa_file("FLUSHALL".to_string());
                }

                if let Some(ref mut stream) = stream {
                    let _ = writeln!(stream, "Success");
                }
            }

            Command::ShowAll => {
                if let Some(ref mut stream) = stream {
                    let _ = writeln!(stream, "{:?}", self.data);
                }
            }
        }
    }

    //STRAIGHT UP BULLSHIT

    // pub fn plotwhole (&mut self, req: Cow<'_, str>, mut stream: TcpStream) {
    //     let req = req.into_owned();
    //     let req = req.trim();
    //     let req_iterator: Vec<String> = req.split(' ').map(|s| s.to_string()).collect();

    //     println!("{:?}", req_iterator);
    //     if req_iterator[0] == "SET" {
    //         if req_iterator.len() != 3 {
    //             return;
    //         }
    //         self.data.insert(req_iterator[1].clone(), req_iterator[2].clone());
    //         if let Ok(_) = stream.write_all(b"value inserted\n") {
    //             println!("Success");
    //         } else {
    //             println!("Err");
    //         }
    //     }

    //     if req_iterator[0] == "GET" {
    //         if req_iterator.len() != 2 {
    //             return;
    //         }
    //         let key = req_iterator[1].clone();
    //         if let Some(value) = self.data.get(&key) {
    //             if let Ok(x) = stream.write_all(value.as_bytes()) {
    //                 stream.write_all(b"\n").unwrap();
    //                 println!("Success")
    //             } else {
    //                 println!("Err")
    //             }
    //         } else {
    //             println!("Coudn't find the data");
    //         }
    //     }

    //     if req_iterator[0] == "DELETE" {
    //         if req_iterator.len() != 2 {
    //             return;
    //         }
    //         let key = req_iterator[1].clone();
    //         if let Some(val) = self.data.remove(&key) {
    //             if let Ok(x) = stream.write_all(val.as_bytes()) {
    //                 stream.write_all(b"\n").unwrap();
    //             }
    //             println!("{}", val);
    //         } else {
    //             println!("didn't delete from db");
    //         }
    //     }

    //     if req_iterator[0] == "FLUSHALL" {
    //         if req_iterator.len() != 1 {
    //             return;
    //         }
    //         self.data.clear();
    //         if let Ok(_) = stream.write_all(b"deleted all records") {
    //             stream.write_all(b"\n").unwrap();
    //             println!("Sucess");
    //             println!("{:?}", self.data);
    //         }
    //     }

    //     if req_iterator[0] == "SHOWALL" {
    //         if req_iterator.len() != 1 {
    //             return;
    //         }
    //         if let Ok(_) = stream.write_all(b"all data") {
    //             stream.write_all(b"\n").unwrap();
    //             println!("{:?}", self.data);
    //         }
    //     }
    // }
}
