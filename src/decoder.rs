use crate::{decoded_inst::InstructionDecoded, error::DecodeError, instructions::*};
use anyhow::{Context, Result};
use compressed::is_compressed;

pub fn decode_rtype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let inst = rtype::RType::new(inst);
    match (inst.funct3(), inst.funct7()) {
        (addi::FUNCT3, addi::FUNCT7) => Ok(InstructionDecoded::Add {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (sub::FUNCT3, sub::FUNCT7) => Ok(InstructionDecoded::Sub {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (sll::FUNCT3, sll::FUNCT7) => Ok(InstructionDecoded::Sll {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (slt::FUNCT3, slt::FUNCT7) => Ok(InstructionDecoded::Slt {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (sltu::FUNCT3, sltu::FUNCT7) => Ok(InstructionDecoded::Sltu {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (xor::FUNCT3, xor::FUNCT7) => Ok(InstructionDecoded::Xor {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (srl::FUNCT3, srl::FUNCT7) => Ok(InstructionDecoded::Srl {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (sra::FUNCT3, sra::FUNCT7) => Ok(InstructionDecoded::Sra {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (or::FUNCT3, or::FUNCT7) => Ok(InstructionDecoded::Or {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (and::FUNCT3, and::FUNCT7) => Ok(InstructionDecoded::And {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat)?,
    }
}

pub fn decode_itype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let iinst = itype::IType::new(inst);
    todo!()
}

pub fn decode_stype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let sinst = stype::SType::new(inst);
    todo!()
}

pub fn decode_utype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let uinst = utype::UType::new(inst);
    todo!()
}

pub fn decode_btype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let binst = btype::BType::new(inst);
    todo!()
}

pub fn decode_jtype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let jinst = jtype::JType::new(inst);
    todo!()
}

pub fn decode(inst: InstructionSize) -> Result<(InstructionDecoded, bool)> {
    if is_compressed(inst) {
        try_decode_compressed(inst).map(|inst| (inst, true))
    } else {
        try_decode(inst).map(|inst| (inst, false))
    }
}

pub fn try_decode(inst: InstructionSize) -> Result<InstructionDecoded> {
    const OPCODE_MASK: InstructionSize = 0b1111111;

    let fmt = match inst & OPCODE_MASK {
        ARITMETIC_REGISTER_MATCH => InstructionFormat::RType,
        STORE_MATCH => InstructionFormat::SType,
        BRANCH_MATCH => InstructionFormat::BType,
        JAL_MATCH => InstructionFormat::JType,
        ARITMETIC_IMMEDIATE_MATCH | FENCE_MATCH | LOAD_MATCH | CSR_MATCH | JALR_MATCH => {
            InstructionFormat::IType
        }
        LUI_MATCH | AUIPC_MATCH => InstructionFormat::UType,
        _ => Err(DecodeError::UnknownInstructionFormat)?,
    };

    let inst = match fmt {
        InstructionFormat::RType => decode_rtype(inst)?,
        InstructionFormat::IType => decode_itype(inst)?,
        InstructionFormat::SType => decode_stype(inst)?,
        InstructionFormat::UType => decode_utype(inst)?,
        InstructionFormat::BType => decode_btype(inst)?,
        InstructionFormat::JType => decode_jtype(inst)?,
    };

    Ok(inst)
}

pub fn try_decode_compressed(_inst: InstructionSize) -> Result<InstructionDecoded> {
    return Err(anyhow::anyhow!(DecodeError::UnknownInstructionFormat))
        .context("Compressed instructions are not supported yet");
}
