pub mod logger;
pub mod scalar;

pub mod builtins;
pub mod data;
mod decode;
pub mod instructions;
pub mod program;
pub mod vlq;

pub use decode::*;
pub use instructions::*;
pub use program::*;

use crate::instructions::FileMarker;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

pub trait InstructionInfo: PartialEq + fmt::Debug + Sized {
    fn to_assembly(&self) -> String;
    fn code() -> InstructionCode;
    fn encode(&self) -> Vec<u8> {
        unimplemented!()
    }
    fn decode(_bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        unimplemented!()
    }
    fn inputs_count(&self) -> usize;
    fn outputs_count(&self) -> usize;
    fn wrap(&self) -> Instruction;
}

#[derive(Debug, PartialEq)]
pub enum DecodingError {
    UnexpectedEOF,
    UnknownInstructionCode(u8),
    ConstantTooLong,
    UTF8Error,
}

#[derive(Debug)]
pub enum InstructionCode {
    NoOperation,

    // Evalution Stack
    PushConst,
    Pop,
    Slice,
    Swap,

    // Data Stack
    Load,
    LoadSequence,
    LoadByIndex,
    LoadSequenceByIndex,

    Store,
    StoreSequence,
    StoreByIndex,
    StoreSequenceByIndex,

    LoadGlobal,
    LoadSequenceGlobal,
    LoadByIndexGlobal,
    LoadSequenceByIndexGlobal,

    StoreGlobal,
    StoreSequenceGlobal,

    Ref,
    RefGlobal,

    LoadByRef,
    LoadSequenceByRef,
    LoadByIndexByRef,
    LoadSequenceByIndexByRef,

    StoreByRef,
    StoreSequenceByRef,
    StoreByIndexByRef,
    StoreSequenceByIndexByRef,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,

    // Boolean
    Not,
    And,
    Or,
    Xor,

    // Comparison
    Lt,
    Le,
    Eq,
    Ne,
    Ge,
    Gt,

    Cast,

    // Flow control
    If,
    Else,
    EndIf,
    LoopBegin,
    LoopEnd,
    Call,
    Return,

    CallBuiltin,

    Assert,
    Dbg,

    Exit,

    FileMarker,
    FunctionMarker,
    LineMarker,
    ColumnMarker,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    NoOperation(NoOperation),

    // Stack
    PushConst(PushConst),
    Pop(Pop),
    Slice(Slice),
    Swap(Swap),

    // Storage
    Load(Load),
    LoadSequence(LoadSequence),
    LoadByIndex(LoadByIndex),
    LoadSequenceByIndex(LoadSequenceByIndex),

    Store(Store),
    StoreSequence(StoreSequence),
    StoreByIndex(StoreByIndex),
    StoreSequenceByIndex(StoreSequenceByIndex),

    LoadGlobal(LoadGlobal),
    LoadSequenceGlobal(LoadSequenceGlobal),
    LoadByIndexGlobal(LoadByIndexGlobal),
    LoadSequenceByIndexGlobal(LoadSequenceByIndexGlobal),

    StoreGlobal(StoreGlobal),
    StoreSequenceGlobal(StoreSequenceGlobal),

    Ref(Ref),
    RefGlobal(RefGlobal),

    LoadByRef(LoadByRef),
    LoadSequenceByRef(LoadSequenceByRef),
    LoadByIndexByRef(LoadByIndexByRef),
    LoadSequenceByIndexByRef(LoadSequenceByIndexByRef),

    StoreByRef(StoreByRef),
    StoreSequenceByRef(StoreSequenceByRef),
    StoreByIndexByRef(StoreByIndexByRef),
    StoreSequenceByIndexByRef(StoreSequenceByIndexByRef),

    // Arithmetic
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Rem(Rem),
    Neg(Neg),

    // Boolean
    Not(Not),
    And(And),
    Or(Or),
    Xor(Xor),

    // Comparison
    Lt(Lt),
    Le(Le),
    Eq(Eq),
    Ne(Ne),
    Ge(Ge),
    Gt(Gt),

    Cast(Cast),

    // Flow control
    If(If),
    Else(Else),
    EndIf(EndIf),
    LoopBegin(LoopBegin),
    LoopEnd(LoopEnd),
    Call(Call),
    Return(Return),

    CallBuiltin(CallBuiltin),

    // Condition utils
    Assert(Assert),
    Dbg(Dbg),

    Exit(Exit),

    FileMarker(FileMarker),
    FunctionMarker(FunctionMarker),
    LineMarker(LineMarker),
    ColumnMarker(ColumnMarker),
}

/// Useful macro to avoid duplicating `match` constructions.
///
/// ```
/// # use zinc_bytecode::{dispatch_instruction, Instruction, InstructionInfo, NoOperation};
/// # use zinc_bytecode::instructions::Add;
/// let ins = NoOperation.wrap();
/// let assembly = dispatch_instruction!(ins => ins.to_assembly());
/// assert_eq!(assembly, "noop");
/// ```
#[macro_export]
macro_rules! dispatch_instruction {
    ($pattern:ident => $expression:expr) => {
        match $pattern {
            Instruction::NoOperation($pattern) => $expression,

            Instruction::PushConst($pattern) => $expression,
            Instruction::Pop($pattern) => $expression,
            Instruction::Slice($pattern) => $expression,
            Instruction::Swap($pattern) => $expression,

            Instruction::Load($pattern) => $expression,
            Instruction::LoadSequence($pattern) => $expression,
            Instruction::LoadByIndex($pattern) => $expression,
            Instruction::LoadSequenceByIndex($pattern) => $expression,

            Instruction::Store($pattern) => $expression,
            Instruction::StoreSequence($pattern) => $expression,
            Instruction::StoreByIndex($pattern) => $expression,
            Instruction::StoreSequenceByIndex($pattern) => $expression,

            Instruction::LoadGlobal($pattern) => $expression,
            Instruction::LoadSequenceGlobal($pattern) => $expression,
            Instruction::LoadByIndexGlobal($pattern) => $expression,
            Instruction::LoadSequenceByIndexGlobal($pattern) => $expression,

            Instruction::StoreGlobal($pattern) => $expression,
            Instruction::StoreSequenceGlobal($pattern) => $expression,

            Instruction::Ref($pattern) => $expression,
            Instruction::RefGlobal($pattern) => $expression,

            Instruction::LoadByRef($pattern) => $expression,
            Instruction::LoadSequenceByRef($pattern) => $expression,
            Instruction::LoadByIndexByRef($pattern) => $expression,
            Instruction::LoadSequenceByIndexByRef($pattern) => $expression,

            Instruction::StoreByRef($pattern) => $expression,
            Instruction::StoreSequenceByRef($pattern) => $expression,
            Instruction::StoreByIndexByRef($pattern) => $expression,
            Instruction::StoreSequenceByIndexByRef($pattern) => $expression,

            Instruction::Add($pattern) => $expression,
            Instruction::Sub($pattern) => $expression,
            Instruction::Mul($pattern) => $expression,
            Instruction::Div($pattern) => $expression,
            Instruction::Rem($pattern) => $expression,
            Instruction::Neg($pattern) => $expression,

            Instruction::Not($pattern) => $expression,
            Instruction::And($pattern) => $expression,
            Instruction::Or($pattern) => $expression,
            Instruction::Xor($pattern) => $expression,

            Instruction::Lt($pattern) => $expression,
            Instruction::Le($pattern) => $expression,
            Instruction::Eq($pattern) => $expression,
            Instruction::Ne($pattern) => $expression,
            Instruction::Ge($pattern) => $expression,
            Instruction::Gt($pattern) => $expression,

            Instruction::Cast($pattern) => $expression,

            Instruction::If($pattern) => $expression,
            Instruction::Else($pattern) => $expression,
            Instruction::EndIf($pattern) => $expression,
            Instruction::LoopBegin($pattern) => $expression,
            Instruction::LoopEnd($pattern) => $expression,
            Instruction::Call($pattern) => $expression,
            Instruction::Return($pattern) => $expression,

            Instruction::CallBuiltin($pattern) => $expression,

            Instruction::Assert($pattern) => $expression,
            Instruction::Dbg($pattern) => $expression,

            Instruction::Exit($pattern) => $expression,
            Instruction::FileMarker($pattern) => $expression,
            Instruction::FunctionMarker($pattern) => $expression,
            Instruction::LineMarker($pattern) => $expression,
            Instruction::ColumnMarker($pattern) => $expression,
        }
    };
}
