use thiserror::Error;

use crate::instructions::InstructionFormat;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
	#[error("Invalid instruction {inst:#08X}")]
	InvalidInstruction { inst: u32 },
	#[error("Invalid instruction format {format:?}")]
    UnknownInstructionFormat {
		format: InstructionFormat
	},
}
