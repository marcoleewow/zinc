//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use zrust_bytecode::Call;
use zrust_bytecode::Exit;
use zrust_bytecode::Instruction;
use zrust_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
type Alias = field;

fn main() {}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
