use crate::{decoded_inst::InstructionDecoded, error::DecodeError, instructions::*};
use anyhow::{Context, Result};

pub fn decode_rtype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let inst = rtype::RType::new(inst);
    match (inst.opcode(), inst.funct3(), inst.funct7()) {
        (ARITMETIC_REGISTER_MATCH, add::FUNCT3, add::FUNCT7) => Ok(InstructionDecoded::Add {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, sub::FUNCT3, sub::FUNCT7) => Ok(InstructionDecoded::Sub {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, sll::FUNCT3, sll::FUNCT7) => Ok(InstructionDecoded::Sll {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, slt::FUNCT3, slt::FUNCT7) => Ok(InstructionDecoded::Slt {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, sltu::FUNCT3, sltu::FUNCT7) => Ok(InstructionDecoded::Sltu {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, xor::FUNCT3, xor::FUNCT7) => Ok(InstructionDecoded::Xor {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, srl::FUNCT3, srl::FUNCT7) => Ok(InstructionDecoded::Srl {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, sra::FUNCT3, sra::FUNCT7) => Ok(InstructionDecoded::Sra {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, or::FUNCT3, or::FUNCT7) => Ok(InstructionDecoded::Or {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, and::FUNCT3, and::FUNCT7) => Ok(InstructionDecoded::And {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown R-Type instruction"),
    }
}

pub fn decode_itype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let iinst = itype::IType::new(inst);
    match (iinst.opcode(), iinst.funct3(), iinst.imm()) {
        (ARITMETIC_IMMEDIATE_MATCH, addi::FUNCT3, _) => Ok(InstructionDecoded::Addi {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, slti::FUNCT3, _) => Ok(InstructionDecoded::Slti {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, sltiu::FUNCT3, _) => Ok(InstructionDecoded::Sltiu {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, xori::FUNCT3, _) => Ok(InstructionDecoded::Xori {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, ori::FUNCT3, _) => Ok(InstructionDecoded::Ori {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, andi::FUNCT3, _) => Ok(InstructionDecoded::Andi {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, slli::FUNCT3, slli::IMM) => Ok(InstructionDecoded::Slli {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, srli::FUNCT3, srli::IMM) => Ok(InstructionDecoded::Srli {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (ARITMETIC_IMMEDIATE_MATCH, srai::FUNCT3, srai::IMM) => Ok(InstructionDecoded::Srai {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (LOAD_MATCH, lb::FUNCT3, _) => Ok(InstructionDecoded::Lb {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (LOAD_MATCH, lh::FUNCT3, _) => Ok(InstructionDecoded::Lh {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (LOAD_MATCH, lw::FUNCT3, _) => Ok(InstructionDecoded::Lw {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (LOAD_MATCH, lbu::FUNCT3, _) => Ok(InstructionDecoded::Lbu {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (LOAD_MATCH, lhu::FUNCT3, _) => Ok(InstructionDecoded::Lhu {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (JALR_MATCH, jalr::FUNCT3, _) => Ok(InstructionDecoded::Jalr {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        (FENCE_MATCH, fence::FUNCT3, _) => todo!(),
        (FENCE_MATCH, fence_i::FUNCT3, _) => todo!(),
        (CSR_MATCH, _, _) => todo!(),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown I-Type instruction"),
    }
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
        // TODO: Support compressed instructions
        // inst if is_compressed(inst) => InstructionFormat::Compressed,
        _ => Err(DecodeError::UnknownInstructionFormat)?,
    };

    let inst = match fmt {
        InstructionFormat::RType => decode_rtype(inst)?,
        InstructionFormat::IType => decode_itype(inst)?,
        InstructionFormat::SType => decode_stype(inst)?,
        InstructionFormat::UType => decode_utype(inst)?,
        InstructionFormat::BType => decode_btype(inst)?,
        InstructionFormat::JType => decode_jtype(inst)?,
        // TODO: Support compressed instructions
        // InstructionFormat::Compressed => decode_compressed(inst)?,
    };

    Ok(inst)
}

pub fn try_decode_compressed(_inst: InstructionSize) -> Result<InstructionDecoded> {
    Err(DecodeError::UnknownInstructionFormat).context("Compressed instructions are not supported yet")
}
