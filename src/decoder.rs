use crate::{decoded_inst::InstructionDecoded, error::DecodeError, instructions::*};
use anyhow::{Context, Result};
use crate::bit_ops::get_bits;

const OPCODE_MASK: InstructionSize = crate::bit_ops::create_mask(7);
// basically the opcode mask but for a compressed instruction (a compresed inst's opcode is the first 2 bits)
const COMPRESSED_MASK: InstructionSize = crate::bit_ops::create_mask(2);

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
        // Mul/div
        (ARITMETIC_REGISTER_MATCH, mul::FUNCT3, mul::FUNCT7) => Ok(InstructionDecoded::Mul {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, mulh::FUNCT3, mulh::FUNCT7) => Ok(InstructionDecoded::Mulh {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, mulsu::FUNCT3, mulsu::FUNCT7) => Ok(InstructionDecoded::Mulsu {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, mulu::FUNCT3, mulu::FUNCT7) => Ok(InstructionDecoded::Mulu {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, div::FUNCT3, div::FUNCT7) => Ok(InstructionDecoded::Div {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, divu::FUNCT3, divu::FUNCT7) => Ok(InstructionDecoded::Divu {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, rem::FUNCT3, rem::FUNCT7) => Ok(InstructionDecoded::Rem {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (ARITMETIC_REGISTER_MATCH, remu::FUNCT3, remu::FUNCT7) => Ok(InstructionDecoded::Remu {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        // Atomic insts
        imm @ (ATOMIC_MATCH, lr_w::FUNCT3, _) if (imm.2 >> 5) == lr_w::FUNCT5 => Ok(InstructionDecoded::LrW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, sc_w::FUNCT3, _) if (imm.2 >> 5) == sc_w::FUNCT5 => Ok(InstructionDecoded::ScW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amoswap_w::FUNCT3, _) if (imm.2 >> 5) == amoswap_w::FUNCT5 => Ok(InstructionDecoded::AmoswapW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amoadd_w::FUNCT3, _) if (imm.2 >> 5) == amoadd_w::FUNCT5 => Ok(InstructionDecoded::AmoaddW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amoxor_w::FUNCT3, _) if (imm.2 >> 5) == amoxor_w::FUNCT5 => Ok(InstructionDecoded::AmoxorW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amoand_w::FUNCT3, _) if (imm.2 >> 5) == amoand_w::FUNCT5 => Ok(InstructionDecoded::AmoandW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amoor_w::FUNCT3, _) if (imm.2 >> 5) == amoor_w::FUNCT5 => Ok(InstructionDecoded::AmoorW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amomin_w::FUNCT3, _) if (imm.2 >> 5) == amomin_w::FUNCT5 => Ok(InstructionDecoded::AmominW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
        }),
        imm @ (ATOMIC_MATCH, amomax_w::FUNCT3, _) if (imm.2 >> 5) == amomax_w::FUNCT5 => Ok(InstructionDecoded::AmomaxW {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
            aq: get_bits(imm.2, 1, 1) == 1,
            rl: get_bits(imm.2, 1, 0) == 1,
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
        imm @ (ARITMETIC_IMMEDIATE_MATCH, slli::FUNCT3, _) if (imm.2 >> 5) == slli::IMM => Ok(InstructionDecoded::Slli {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: imm.2 & 0b11111,
        }),
        imm @ (ARITMETIC_IMMEDIATE_MATCH, srli::FUNCT3, _) if (imm.2 >> 5) == srli::IMM => Ok(InstructionDecoded::Srli {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: imm.2 & 0b11111,
        }),
        imm @ (ARITMETIC_IMMEDIATE_MATCH, srai::FUNCT3, _) if (imm.2 >> 5) == srai::IMM => Ok(InstructionDecoded::Srai {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: imm.2 & 0b11111,
        }),
        // Load
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
        (FENCE_MATCH, fence::FUNCT3, _) => {
            let pred = iinst.imm() & 0b1111;
            let succ = (iinst.imm() >> 4) & 0b1111;
            Ok(InstructionDecoded::Fence { pred, succ })
        }
        (FENCE_MATCH, fence_i::FUNCT3, _) => {
            let pred = get_bits(iinst.imm(), 4, 0);
            let succ = get_bits(iinst.imm(), 4, 4);
            Ok(InstructionDecoded::FenceI { pred, succ })
        }
        (CSR_MATCH, csrrw::FUNCT3, _) => Ok(InstructionDecoded::CsrRw {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.uimm(),
        }),
        (CSR_MATCH, csrrs::FUNCT3, _) => Ok(InstructionDecoded::CsrRs {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.uimm(),
        }),
        (CSR_MATCH, csrrc::FUNCT3, _) => Ok(InstructionDecoded::CsrRc {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.uimm(),
        }),
        (CSR_MATCH, csrrwi::FUNCT3, _) => Ok(InstructionDecoded::CsrRwi {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.uimm(),
        }),
        (CSR_MATCH, csrrsi::FUNCT3, _) => Ok(InstructionDecoded::CsrRsi {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.uimm(),
        }),
        (CSR_MATCH, csrrci::FUNCT3, _) => Ok(InstructionDecoded::CsrRci {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.uimm(),
        }),
        // e-insts (ebreak, ecall)
        (CSR_MATCH, ebreak::FUNCT3, ebreak::IMM) => Ok(InstructionDecoded::EBreak),
        (CSR_MATCH, ecall::FUNCT3, ecall::IMM) => Ok(InstructionDecoded::ECall),
        (CSR_MATCH, mret::FUNCT3, mret::IMM) => Ok(InstructionDecoded::MRet),
        (CSR_MATCH, sret::FUNCT3, sret::IMM) => Ok(InstructionDecoded::SRet),
        // TODO: SFenceVMA
        // (CSR_MATCH, sfencevma::FUNCT3, sfencevma::IMM) => Ok(InstructionDecoded::SFenceVma),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown I-Type instruction"),
    }
}

pub fn decode_stype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let sinst = stype::SType::new(inst);
    match sinst.funct3() {
        sb::FUNCT3 => Ok(InstructionDecoded::Sb {
            rs1: sinst.rs1(),
            rs2: sinst.rs2(),
            imm: sinst.imm(),
        }),
        sh::FUNCT3 => Ok(InstructionDecoded::Sh {
            rs1: sinst.rs1(),
            rs2: sinst.rs2(),
            imm: sinst.imm(),
        }),
        sw::FUNCT3 => Ok(InstructionDecoded::Sw {
            rs1: sinst.rs1(),
            rs2: sinst.rs2(),
            imm: sinst.imm(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown S-Type instruction"),
    }
}

pub fn decode_utype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let uinst = utype::UType::new(inst);
    match uinst.opcode() {
        LUI_MATCH => Ok(InstructionDecoded::Lui {
            rd: uinst.rd(),
            imm: uinst.imm(),
        }),
        AUIPC_MATCH => Ok(InstructionDecoded::AuiPc {
            rd: uinst.rd(),
            imm: uinst.imm(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown U-Type instruction"),
    }
}

pub fn decode_btype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let binst = btype::BType::new(inst);
    match binst.funct3() {
        beq::FUNCT3 => Ok(InstructionDecoded::Beq {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bne::FUNCT3 => Ok(InstructionDecoded::Bne {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        blt::FUNCT3 => Ok(InstructionDecoded::Blt {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bge::FUNCT3 => Ok(InstructionDecoded::Bge {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bltu::FUNCT3 => Ok(InstructionDecoded::Bltu {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bgeu::FUNCT3 => Ok(InstructionDecoded::Bgeu {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown B-Type instruction"),
    }
}

pub fn decode_jtype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let jinst = jtype::JType::new(inst);
    match jinst.opcode() {
        JAL_MATCH => Ok(InstructionDecoded::Jal {
            rd: jinst.rd(),
            imm: jinst.imm(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown J-Type instruction"),
    }
}

pub fn try_decode(inst: InstructionSize) -> Result<InstructionDecoded> {
    // if its a compressed inst then dont bother with regular decoding, instead decode it as compressed and return the result
    match inst & COMPRESSED_MASK {
        // its a compressed instruction
        0 | 1 | 2 => return try_decode_compressed(inst),
        // otherwise just continue with regular decoding
        _ => (),
    }

    let fmt = match inst & OPCODE_MASK {
        ARITMETIC_REGISTER_MATCH => InstructionFormat::RType,
        STORE_MATCH => InstructionFormat::SType,
        BRANCH_MATCH => InstructionFormat::BType,
        JAL_MATCH => InstructionFormat::JType,
        ARITMETIC_IMMEDIATE_MATCH | FENCE_MATCH | LOAD_MATCH | CSR_MATCH | JALR_MATCH => {
            InstructionFormat::IType
        }
        LUI_MATCH | AUIPC_MATCH => InstructionFormat::UType,
        _ => Err(DecodeError::UnknownInstructionFormat).context(format!("Failed to decode inst {inst}"))?,
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
    Err(DecodeError::UnknownInstructionFormat).context(format!("Compressed instructions are not supported yet"))
}
