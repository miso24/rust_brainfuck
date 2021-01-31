use brainfuck::parser::{parse, Instruction};
use brainfuck::lexer::lex;
use std::collections::HashMap;

macro_rules! parse_code {
    ($code:expr) => {
        {
            let tokens = lex($code).unwrap();
            parse(&tokens)
        }
    };
}

macro_rules! assert_eq_program {
    ($prog:expr, $insts:expr, $lbr:expr, $rbr:expr) => {{
        assert_eq!($prog.insts, $insts);
        assert_eq!($prog.lbr_pos, $lbr);
        assert_eq!($prog.rbr_pos, $rbr);
    }};
}

#[test]
fn parse_simple_code() {
    let code = ">+-<";
    let prog = parse_code!(code).unwrap();

    let insts = vec![
        Instruction::AddPtr(1),
        Instruction::Add(1),
        Instruction::Sub(1),
        Instruction::SubPtr(1),
    ];
    let lbr_pos = HashMap::new();
    let rbr_pos = HashMap::new();

    assert_eq_program!(prog, insts, lbr_pos, rbr_pos)
}

#[test]
fn parse_complex_code() {
    let code = ">++++++[<++++++++>-].";
    let prog = parse_code!(code).unwrap();

    let insts = vec![
        Instruction::AddPtr(1),
        Instruction::Add(6),
        Instruction::JumpToRBracket(0),
        Instruction::SubPtr(1),
        Instruction::Add(8),
        Instruction::AddPtr(1),
        Instruction::Sub(1),
        Instruction::JumpToLBracket(0),
        Instruction::Write,
    ];
    let mut lbr_pos = HashMap::new();
    let mut rbr_pos = HashMap::new();

    lbr_pos.insert(0, 2);
    rbr_pos.insert(0, 7);

    assert_eq_program!(prog, insts, lbr_pos, rbr_pos)
}

#[test]
fn parse_bracket() {
    let code = "++[>++[>+++[>+<-]<-]<-]";
    let prog = parse_code!(code).unwrap();

    let insts = vec![
        Instruction::Add(2),
        Instruction::JumpToRBracket(0),
        Instruction::AddPtr(1),
        Instruction::Add(2),
        Instruction::JumpToRBracket(1),
        Instruction::AddPtr(1),
        Instruction::Add(3),
        Instruction::JumpToRBracket(2),
        Instruction::AddPtr(1),
        Instruction::Add(1),
        Instruction::SubPtr(1),
        Instruction::Sub(1),
        Instruction::JumpToLBracket(2),
        Instruction::SubPtr(1),
        Instruction::Sub(1),
        Instruction::JumpToLBracket(1),
        Instruction::SubPtr(1),
        Instruction::Sub(1),
        Instruction::JumpToLBracket(0),
    ];
    let mut lbr_pos = HashMap::new();
    let mut rbr_pos = HashMap::new();

    lbr_pos.insert(0, 1);
    lbr_pos.insert(1, 4);
    lbr_pos.insert(2, 7);
    rbr_pos.insert(0, 18);
    rbr_pos.insert(1, 15);
    rbr_pos.insert(2, 12);

    assert_eq_program!(prog, insts, lbr_pos, rbr_pos)
}

#[test]
fn unclosed_bracket() {
    let code = "[[[]]";
    let prog = parse_code!(code);

    assert!(prog.is_err())
}
