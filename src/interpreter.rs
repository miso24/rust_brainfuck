use crate::lexer::lex;
use crate::parser::{parse, Program, Instruction};
use std::io::{stdin, Read};
use std::fmt;

pub struct Interpreter {
    stack: Vec<u8>,
    sp: usize,
    stack_size: usize,
}

#[derive(Debug)]
pub enum InterpreterError {
    MemoryError,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::MemoryError => write!(f, "Memory error")
        }
    }
}

impl std::error::Error for InterpreterError {}

impl Interpreter {
    pub fn new(stack_size: usize) -> Self {
        Self {
            stack: vec![0; stack_size],
            sp: 0,
            stack_size,
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tokens = lex(code)?;
        let prog = parse(&tokens)?;

        self.exec(&prog)?;
        Ok(())
    }

    fn exec(&mut self, prog: &Program) -> Result<(), InterpreterError> {
        let mut pos = 0;

        while pos < prog.insts.len() {
            match prog.insts[pos] {
                Instruction::Add(n) => {
                    let (new_val, _) = self.stack[self.sp].overflowing_add(n);
                    self.stack[self.sp] = new_val;
                },
                Instruction::Sub(n) => {
                    let (new_val, _) = self.stack[self.sp].overflowing_sub(n);
                    self.stack[self.sp] = new_val;
                },
                Instruction::AddPtr(n) => {
                    let new_sp = self.sp + n as usize;
                    if new_sp >= self.stack_size {
                        return Err(InterpreterError::MemoryError);
                    }
                    self.sp = new_sp;
                },
                Instruction::SubPtr(n) => {
                    match self.sp.checked_sub(n as usize) {
                        Some(n) => self.sp = n,
                        None => return Err(InterpreterError::MemoryError),
                    }
                },
                Instruction::Read => {
                    let mut buffer = [0; 1];
                    stdin().read_exact(&mut buffer).unwrap();
                    self.stack[self.sp] = buffer[0];
                },
                Instruction::Write => print!("{}", self.stack[self.sp] as char),
                Instruction::JumpToLBracket(idx) => {
                    if self.stack[self.sp] != 0 {
                        let entry = prog.lbr_pos.get(&idx).unwrap();
                        pos = *entry;
                    }
                },
                Instruction::JumpToRBracket(idx) => {
                    if self.stack[self.sp] == 0 {
                        let entry = prog.rbr_pos.get(&idx).unwrap();
                        pos = *entry;
                    }
                },
            }
            pos += 1;
        }

        Ok(())
    }
}
