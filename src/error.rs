#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnknownInstruction(String),
    UnknownInstructionFormat,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DecodeError::UnknownInstruction(inst) => write!(f, "Unknown instruction: {inst}"),
            DecodeError::UnknownInstructionFormat => write!(f, "Unknown instruction format"),
        }
    }
}

impl std::error::Error for DecodeError {}
