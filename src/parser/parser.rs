#![allow(dead_code)]

// ["SET", "count", "0"]


use std::{collections::HashMap, str};

#[derive(Debug)]
pub enum Instruction {
    GET(String),
    SET(String, String),
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
        // let toks: Vec<String> = self.src.split([' ', '\n']).map(String::from).collect();
        // dbg!(&self.get_token()[..] == "SET");
        // return;
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
                        "KEYS" => {
                            let inst = Instruction::KEYS;
                            self.instructions.push(inst);
                        }
                        "FLUSHALL" => {
                            let inst = Instruction::FLUSHALL;
                            self.instructions.push(inst);
                        }
                        default => {}
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

    pub fn interprete(&mut self, ht: &mut HashMap<String, String>) {
        for inst in &self.lexer.instructions {
            match inst {
                Instruction::GET(key) => {
                    dbg!(&ht.get(key));
                }
                Instruction::SET(key, val) => {
                    ht.insert(key.clone(), val.clone());
                }
                Instruction::KEYS => {
                    dbg!(&ht);
                }
                Instruction::FLUSHALL => {
                    ht.clear();
                }
            }
        }
    }
}








//
