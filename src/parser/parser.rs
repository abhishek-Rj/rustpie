#![allow(dead_code)]
use std::{collections::HashMap, num::ParseIntError, str};

#[derive(Debug)]
pub enum Instruction {
    GET(String),
    SET(String, String),
    EXISTS(String),
    MGET(Vec<String>),
    INCR(String),
    STRLEN(String),
    KEYS,
    FLUSHALL
}

#[derive(Debug)]
pub struct Lexer {
    pub instructions: Vec<Instruction>,
    _src: String,
    toks: Vec<String>,
    pos: usize
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            instructions: Vec::new(),
            _src: src.to_string(),
            toks: src.split([' ', '\n']).map(String::from).collect(),
            pos: 0
        }
    }

    pub fn peek_token(&self) -> Option<&String> {
        if self.toks.len() > self.pos {
            Some(&self.toks[self.pos])
        } else {
            None
        }
    }

    pub fn get_token(&mut self) -> &String {
        let tok = &self.toks[self.pos];
        self.pos += 1;
        tok
    }

    pub fn tokenize(&mut self) {
        loop {
            match self.peek_token() {
                None => break,
                Some(_) => {
                    match &self.get_token()[..] {
                        "SET" => {
                            let inst = Instruction::SET(self.get_token().clone(), self.get_token().clone());
                            self.instructions.push(inst);
                        }
                        "GET" => {
                            let inst = Instruction::GET(self.get_token().clone());
                            self.instructions.push(inst);
                        }
                        "EXISTS" => {
                            let inst = Instruction::EXISTS(self.get_token().clone());
                            self.instructions.push(inst);
                        }
                        "MGET" => {
                            let mut keys: Vec<String> = Vec::new();
                            while self.peek_token() != None {
                                keys.push(self.get_token().clone());
                            }
                            let inst = Instruction::MGET(keys);
                            self.instructions.push(inst);
                        }
                        "INCR" => {
                            let inst = Instruction::INCR(self.get_token().clone());
                            self.instructions.push(inst);
                        }
                        "STRLEN" => {
                            let inst = Instruction::STRLEN(self.get_token().clone());
                            self.instructions.push(inst);
                        }
                        "KEYS" => {
                            let inst = Instruction::KEYS;
                            self.instructions.push(inst);
                        }
                        "FLUSHALL" => {
                            let inst = Instruction::FLUSHALL;
                            self.instructions.push(inst);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub struct Interpreter {
    pub lexer: Lexer
}

impl Interpreter {
    pub fn new(src: &str) -> Self {
        Self {
            lexer: Lexer::new(src)
        }
    }

    pub fn interprete(&mut self, ht: &mut HashMap<String, String>) -> String {
        for inst in &self.lexer.instructions {
            match inst {
                Instruction::GET(key) => {
                    if let Some(ret) =  ht.get(key) {
                        return ret.clone();
                    } else {
                        panic!();
                    };
                }
                Instruction::SET(key, val) => {
                    ht.insert(key.clone(), val.clone());
                    return "OK".to_string();
                }
                Instruction::MGET(keys) => {
                    let mut vals: Vec<String> = Vec::new();
                    for key in keys {
                        if let Some(ret) =  ht.get(key) {
                            vals.push(ret.clone());
                        } else {
                            panic!();
                        };
                    }
                    return format!("{:?}", vals);
                }

                Instruction::INCR(key) => {
                    // if let Ok(ret) = ht.get(key).parse::<i32>() {
                    //     let val = ht.get_mut(key).unwrap();
                    //     *val = format!("{}", ret + 1);
                    //     return format!("{val}");
                    // } else{
                    //     let val = ht.get(key).unwrap();
                    //     return format!("Value |{val}| is not a valid integer!");
                    // }

                    match ht.get(key).unwrap().parse::<i32>() {
                        Ok(ret) => {
                            let val = ht.get_mut(key).unwrap();
                            *val = format!("{}", ret + 1);
                            return format!("{val}");
                        }
                        Err(err) => {
                            return format!("Value |{err}| is not a valid integer!");
                        }
                    }
                }

                Instruction::STRLEN(key) => {
                }
                Instruction::EXISTS(key) => {
                    if let Some(_) = ht.get(key) {
                        return "true".to_string()
                    } else {
                        return "false".to_string()
                    }
                }
                Instruction::KEYS => {
                    return format!("{:?}", &ht);
                }
                Instruction::FLUSHALL => {
                    ht.clear();
                    return "OK".to_string();
                }
            }
        }
        "ERROR: Something went wrong!".to_string()
    }
}








//
