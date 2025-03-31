use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
	#[error("Invalid instruction {0:#08X}")]
	InvalidInstruction(u32),
}
