use crate::parser::plotwhole::Value;

pub fn provide_value(req: &str, req_value: &str) -> Result<Value, String> {
    if req == "String" {
        Ok(Value::String(req_value.to_string()))
    } else if req == "Integer" {
        if let Ok(value) = req_value.parse::<i64>() {
            Ok(Value::Integer(value))
        } else {
            Ok(Value::String(req_value.to_string()))
        }
    } else if req == "Float" {
        if let Ok(value) = req_value.parse::<f64>() {
            Ok(Value::Float(value))
        } else {
            Ok(Value::String(req_value.to_string()))
        }
    } else if req == "Boolean" {
        if let Ok(value) = req_value.parse::<bool>() {
            Ok(Value::Boolean(value))
        } else {
            Ok(Value::String(req_value.to_string()))
        }
    } else {
        Err("Not a valid request".into())
    }
}
