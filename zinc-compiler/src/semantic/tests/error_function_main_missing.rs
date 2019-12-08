//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::semantic::BinaryAnalyzer;
use crate::semantic::Error as SemanticError;
use crate::syntax::Parser;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn another() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionMainMissing));

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
