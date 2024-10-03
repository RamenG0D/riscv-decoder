use std::fmt::{Display, Formatter, Result};

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnknownInstruction,
    UnknownInstructionFormat,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
