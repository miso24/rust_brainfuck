use crate::lexer::Token;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(u8),
    Sub(u8),
    AddPtr(u8),
    SubPtr(u8),
    Write,
    Read,
    JumpToLBracket(usize),
    JumpToRBracket(usize),
}

#[derive(Debug)]
pub enum ParseError {
    UnclosedBracket,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnclosedBracket => write!(f, "Unclosed bracket"),
        }
    }
}

impl std::error::Error for ParseError {}

pub struct Program {
    pub insts: Vec<Instruction>,
    pub lbr_pos: HashMap<usize, usize>,
    pub rbr_pos: HashMap<usize, usize>,
}

impl Program {
    fn new() -> Self {
        Self {
            insts: Vec::new(),
            lbr_pos: HashMap::new(),
            rbr_pos: HashMap::new(),
        }
    }

    fn add_instruction(&mut self, inst: Instruction) {
        self.insts.push(inst);
    }

    fn add_rbracket(&mut self, id: usize, pos: usize) {
        self.rbr_pos.insert(id, pos);
    }

    fn add_lbracket(&mut self, id: usize, pos: usize) {
        self.lbr_pos.insert(id, pos);
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Program, ParseError> {
    let mut pos = 0;
    let mut inst_pos = 0;
    let mut bracket_stack = Vec::new();
    let mut bracket_id = 0;

    let mut program = Program::new();

    macro_rules! append_opt_instruction {
        ($tok_kind:expr, $op_kind:expr) => {{
            let (num, _pos) = consume_many_tokens(&tokens, pos, $tok_kind);
            program.add_instruction($op_kind(num));
            pos = _pos;
        }};
    }

    macro_rules! append_instruction {
        ($op_kind:expr) => {{
            program.add_instruction($op_kind);
            pos += 1;
        }};
    }

    while pos < tokens.len() {
        match tokens[pos] {
            Token::Plus => append_opt_instruction!(Token::Plus, Instruction::Add),
            Token::Minus => append_opt_instruction!(Token::Minus, Instruction::Sub),
            Token::Greater => append_opt_instruction!(Token::Greater, Instruction::SubPtr),
            Token::Less => append_opt_instruction!(Token::Less, Instruction::AddPtr),
            Token::Period => append_instruction!(Instruction::Write),
            Token::Comma => append_instruction!(Instruction::Read),
            Token::LBracket => {
                program.add_lbracket(bracket_id, inst_pos);
                bracket_stack.push(bracket_id);
                program.add_instruction(Instruction::JumpToRBracket(bracket_id));
                bracket_id += 1;
                pos += 1;
            }
            Token::RBracket => {
                match bracket_stack.pop() {
                    Some(id) => {
                        program.add_rbracket(id, inst_pos);
                        program.add_instruction(Instruction::JumpToLBracket(id));
                    }
                    None => return Err(ParseError::UnclosedBracket),
                }
                pos += 1;
            }
        }
        inst_pos += 1;
    }

    if !bracket_stack.is_empty() {
        Err(ParseError::UnclosedBracket)
    } else {
        Ok(program)
    }
}

fn consume_many_tokens(tokens: &Vec<Token>, mut pos: usize, t: Token) -> (u8, usize) {
    let mut counter = 0;
    while pos < tokens.len() && tokens[pos] == t && counter < 255 {
        counter += 1;
        pos += 1;
    }
    (counter, pos)
}
