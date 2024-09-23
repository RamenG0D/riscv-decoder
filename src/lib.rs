pub mod decoded_inst;
pub mod decoder;
pub mod error;
pub mod instructions;

pub mod bit_ops {
    pub use bit_ops::bitops_u32::*;

    pub fn zero_extend(inst: u32) -> u32 {
        clear_bit(inst, 31)
    }
}
