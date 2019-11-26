//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zrust_bytecode::Call;
use zrust_bytecode::Copy;
use zrust_bytecode::Exit;
use zrust_bytecode::FrameBegin;
use zrust_bytecode::FrameEnd;
use zrust_bytecode::Instruction;
use zrust_bytecode::Push;
use zrust_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut inner = 25;
    {
        inner = 50;
    }
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Push(Push::new(BigInt::from(25), false, 8)),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(50), false, 8)),
        Instruction::Copy(Copy::new(1)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
