use crate::parser::{plotwhole::Command, provide_value::provide_value};

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
            if let Ok(value) = value {
                Ok(Command::Set(parts[1].to_string(), data_type.to_string(), value))
            } else {
                Err("Not a valid data type".into())
            }
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

