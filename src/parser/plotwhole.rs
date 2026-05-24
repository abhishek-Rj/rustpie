use crate::{persistent_storage::add_data_toa_file};
use std::fmt;
use std::{borrow::Cow, collections::HashMap, io::Write, net::TcpStream};
use crate::parser::parse_command::parse_command;

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
}
