
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnknownInstruction(String),
    UnknownInstructionFormat(String),
    NotCompressedInstruction,
}
