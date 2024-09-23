#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnknownInstruction,
    UnknownInstructionFormat,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DecodeError::UnknownInstruction => write!(f, "Unknown instruction"),
            DecodeError::UnknownInstructionFormat => write!(f, "Unknown instruction format"),
        }
    }
}

impl std::error::Error for DecodeError {}
