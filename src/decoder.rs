use crate::bit_ops::*;
use crate::{decoded_inst::InstructionDecoded, error::DecodeError, instructions::*};
use anyhow::{Context, Result};
use paste::paste;

const OPCODE_MASK: InstructionSize = crate::bit_ops::create_mask(7);
// basically the opcode mask but for a compressed instruction (a compresed inst's opcode is the first 2 bits)
const COMPRESSED_MASK: InstructionSize = crate::bit_ops::create_mask(2);

pub fn decode_rtype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let inst = rtype::RType::new(inst);
    match inst.opcode() {
        ARITMETIC_REGISTER_MATCH => {
            match (inst.funct3(), inst.funct7()) {
                (add::FUNCT3, add::FUNCT7) => Ok(InstructionDecoded::Add {
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

                // M extension
                (mul::FUNCT3, mul::FUNCT7) => Ok(InstructionDecoded::Mul {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (mulh::FUNCT3, mulh::FUNCT7) => Ok(InstructionDecoded::Mulh {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (mulu::FUNCT3, mulu::FUNCT7) => Ok(InstructionDecoded::Mulu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (mulsu::FUNCT3, mulsu::FUNCT7) => Ok(InstructionDecoded::Mulsu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (div::FUNCT3, div::FUNCT7) => Ok(InstructionDecoded::Div {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (divu::FUNCT3, divu::FUNCT7) => Ok(InstructionDecoded::Divu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (rem::FUNCT3, rem::FUNCT7) => Ok(InstructionDecoded::Rem {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (remu::FUNCT3, remu::FUNCT7) => Ok(InstructionDecoded::Remu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),

                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Arithmetic Register instruction (R-type)"),
            }
        }
        ATOMIC_MATCH => {
            let funct5 = get_bits(inst.funct7(), 5, 2);
            let rl = is_set(inst.funct7(), 0);
            let aq = is_set(inst.funct7(), 1);
            match (inst.funct3(), funct5) {
                (amoswap_w::FUNCT3, amoswap_w::FUNCT5) => Ok(InstructionDecoded::AmoswapW {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                    rl, aq,
                }),
                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Atomic instruction"),
            }
        }
        FLOATING_POINT_MATCH => {
            let funct5 = get_bits(inst.funct7(), 5, 2);
            let fmt = get_bits(inst.funct7(), 2, 0) as u8;
            // assert!(fmt == 0, "the fmt of an inst cannot be non 0 because we only support single precision floating point instructions currently!");
            const SINGLE_PRECISION_MATCH: u8 = 0;
            const DOUBLE_PRECISION_MATCH: u8 = 1;
            const QUAD_PRECISION_MATCH: u8 = 3;

            match fmt {
                SINGLE_PRECISION_MATCH => decode_single_precision(&inst, inst.funct3(), funct5),
                DOUBLE_PRECISION_MATCH => decode_double_precision(&inst, inst.funct3(), funct5),
                QUAD_PRECISION_MATCH => decode_quad_precision(&inst, inst.funct3(), funct5),
                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Floating Point instruction"),
            }
        }

        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown R-Type instruction"),
    }
}

fn decode_single_precision(inst: &rtype::RType, funct3: InstructionSize, funct5: InstructionSize) -> Result<InstructionDecoded> {
    match (funct3, funct5) {
        (fadd_s::FUNCT3, fadd_s::FUNCT5) => Ok(InstructionDecoded::FaddS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsub_s::FUNCT3, fsub_s::FUNCT5) => Ok(InstructionDecoded::FsubS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmul_s::FUNCT3, fmul_s::FUNCT5) => Ok(InstructionDecoded::FmulS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fdiv_s::FUNCT3, fdiv_s::FUNCT5) => Ok(InstructionDecoded::FdivS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnj_s::FUNCT3, fsgnj_s::FUNCT5) => Ok(InstructionDecoded::FsgnjS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjn_s::FUNCT3, fsgnjn_s::FUNCT5) => Ok(InstructionDecoded::FsgnjnS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjx_s::FUNCT3, fsgnjx_s::FUNCT5) => Ok(InstructionDecoded::FsgnjxS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmin_s::FUNCT3, fmin_s::FUNCT5) => Ok(InstructionDecoded::FminS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmax_s::FUNCT3, fmax_s::FUNCT5) => Ok(InstructionDecoded::FmaxS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fcvt_w_s::FUNCT3, fcvt_w_s::FUNCT5) => match inst.rs2() {
            fcvt_w_s::RS2 => Ok(InstructionDecoded::FcvtWUS {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            fcvt_wu_s::RS2 => Ok(InstructionDecoded::FcvtWS {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Floating Point instruction"),
        }
        (feq_s::FUNCT3, feq_s::FUNCT5) => Ok(InstructionDecoded::FeqS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (flt_s::FUNCT3, flt_s::FUNCT5) => Ok(InstructionDecoded::FltS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fle_s::FUNCT3, fle_s::FUNCT5) => Ok(InstructionDecoded::FleS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fclass_s::FUNCT3, fclass_s::FUNCT5) => Ok(InstructionDecoded::FClassS {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (fcvt_s_w::FUNCT3, fcvt_s_w::FUNCT5) => Ok(InstructionDecoded::FcvtSW {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (fcvt_s_wu::FUNCT3, fcvt_s_wu::FUNCT5) => Ok(InstructionDecoded::FcvtSWU {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (fmv_x_w::FUNCT3, fmv_x_w::FUNCT5) => Ok(InstructionDecoded::FmvXW {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (fmv_w_x::FUNCT3, fmv_w_x::FUNCT5) => Ok(InstructionDecoded::FmvWX {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Single Precision Floating Point instruction"),
    }
}
fn decode_double_precision(inst: &rtype::RType, funct3: InstructionSize, funct5: InstructionSize) -> Result<InstructionDecoded> {
    match (funct3, funct5) {
        (fadd_d::FUNCT3, fadd_d::FUNCT5) => Ok(InstructionDecoded::FaddD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsub_d::FUNCT3, fsub_d::FUNCT5) => Ok(InstructionDecoded::FsubD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmul_d::FUNCT3, fmul_d::FUNCT5) => Ok(InstructionDecoded::FmulD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fdiv_d::FUNCT3, fdiv_d::FUNCT5) => Ok(InstructionDecoded::FdivD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnj_d::FUNCT3, fsgnj_d::FUNCT5) => Ok(InstructionDecoded::FsgnjD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjn_d::FUNCT3, fsgnjn_d::FUNCT5) => Ok(InstructionDecoded::FsgnjnD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjx_d::FUNCT3, fsgnjx_d::FUNCT5) => Ok(InstructionDecoded::FsgnjxD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmin_d::FUNCT3, fmin_d::FUNCT5) => Ok(InstructionDecoded::FminD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmax_d::FUNCT3, fmax_d::FUNCT5) => Ok(InstructionDecoded::FmaxD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fcvt_s_d::FUNCT3, fcvt_s_d::FUNCT5) => match inst.rs2() {
            fcvt_s_d::RS2 => Ok(InstructionDecoded::FcvtSD {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            fcvt_d_s::RS2 => Ok(InstructionDecoded::FcvtDS {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Floating Point instruction"),
        }
        (feq_d::FUNCT3, feq_d::FUNCT5) => Ok(InstructionDecoded::FeqD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (flt_d::FUNCT3, flt_d::FUNCT5) => Ok(InstructionDecoded::FltD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fle_d::FUNCT3, fle_d::FUNCT5) => Ok(InstructionDecoded::FleD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fclass_d::FUNCT3, fclass_d::FUNCT5) => Ok(InstructionDecoded::FClassD {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        // (fcvt_w_d::FUNCT3, fcvt_w_d::FUNCT5) => Ok(InstructionDecoded::FcvtWD {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        // (fcvt_wu_d::FUNCT3, fcvt_wu_d::FUNCT5) => Ok(InstructionDecoded::FcvtWUD {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        // (fcvt_d_w::FUNCT3, fcvt_d_w::FUNCT5) => Ok(InstructionDecoded::FcvtDW {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        // (fcvt_d_wu::FUNCT3, fcvt_d_wu::FUNCT5) => Ok(InstructionDecoded::FcvtDUW {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        // (fmv_x_d::FUNCT3, fmv_x_d::FUNCT5) => Ok(InstructionDecoded::FmvXD {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        // (fmv_d_x::FUNCT3, fmv_d_x::FUNCT5) => Ok(InstructionDecoded::FmvDX {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Double Precision Floating Point instruction"),
    }
}
fn decode_quad_precision(_inst: &rtype::RType, _funct3: InstructionSize, _funct5: InstructionSize) -> Result<InstructionDecoded> {
    return Err(DecodeError::UnknownInstructionFormat).context("Quad Precision Floating Point instructions are not supported yet");
}

pub fn decode_itype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let iinst = itype::IType::new(inst);
    match iinst.opcode() {
        ARITMETIC_IMMEDIATE_MATCH => {
            let funct5 = get_bits(iinst.imm(), 2, 5);
            let imm = get_bits(iinst.imm(), 5, 0);
            match (iinst.funct3(), funct5) {
                (addi::FUNCT3, _) => Ok(InstructionDecoded::Addi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (slti::FUNCT3, _) => Ok(InstructionDecoded::Slti {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (sltiu::FUNCT3, _) => Ok(InstructionDecoded::Sltiu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (xori::FUNCT3, _) => Ok(InstructionDecoded::Xori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (ori::FUNCT3, _) => Ok(InstructionDecoded::Ori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (andi::FUNCT3, _) => Ok(InstructionDecoded::Andi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (slli::FUNCT3, slli::IMM) => {
                    Ok(InstructionDecoded::Slli {
                        rd: iinst.rd(),
                        rs1: iinst.rs1(),
                        imm
                    })
                }
                (srli::FUNCT3, srli::IMM) => {
                    Ok(InstructionDecoded::Srli {
                        rd: iinst.rd(),
                        rs1: iinst.rs1(),
                        imm,
                    })
                }
                (srai::FUNCT3, srai::IMM) => {
                    Ok(InstructionDecoded::Srai {
                        rd: iinst.rd(),
                        rs1: iinst.rs1(),
                        imm,
                    })
                },
                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Arithmetic immediate I-Type instruction")
            }
        }
        // Load
        LOAD_MATCH => match iinst.funct3() {
            lb::FUNCT3 => Ok(InstructionDecoded::Lb {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lh::FUNCT3 => Ok(InstructionDecoded::Lh {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lw::FUNCT3 => Ok(InstructionDecoded::Lw {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lbu::FUNCT3 => Ok(InstructionDecoded::Lbu {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lhu::FUNCT3 => Ok(InstructionDecoded::Lhu {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Load I-Type instruction")
        }
        FLOAD_MATCH => match iinst.funct3() {
            flw::FUNCT3 => Ok(InstructionDecoded::Flw {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            fld::FUNCT3 => Ok(InstructionDecoded::Fld {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Float Load I-Type instruction")
        }
        JALR_MATCH if iinst.funct3() == jalr::FUNCT3 => Ok(InstructionDecoded::Jalr {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        FENCE_MATCH => {
            let pred = get_bits(iinst.imm(), 4, 0);
            let succ = get_bits(iinst.imm(), 4, 4);
            match iinst.funct3() {
                fence::FUNCT3 => Ok(InstructionDecoded::Fence { pred, succ }),
                fence_i::FUNCT3 => Ok(InstructionDecoded::FenceI { pred, succ }),
                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Fence I-Type instruction")
            }
        }
        CSR_MATCH => {
            match (iinst.funct3(), iinst.imm()) {
                (csrrw::FUNCT3, _) => Ok(InstructionDecoded::CsrRw {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrs::FUNCT3, _) => Ok(InstructionDecoded::CsrRs {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrc::FUNCT3, _) => Ok(InstructionDecoded::CsrRc {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrwi::FUNCT3, _) => Ok(InstructionDecoded::CsrRwi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrsi::FUNCT3, _) => Ok(InstructionDecoded::CsrRsi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrci::FUNCT3, _) => Ok(InstructionDecoded::CsrRci {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                // e-insts (ebreak, ecall)
                (sfencevma::FUNCT3, sfencevma::IMM) => Ok(InstructionDecoded::SFenceVma),
                (ebreak::FUNCT3, ebreak::IMM) => Ok(InstructionDecoded::EBreak),
                (ecall::FUNCT3, ecall::IMM) => Ok(InstructionDecoded::ECall),
                (mret::FUNCT3, mret::IMM) => Ok(InstructionDecoded::MRet),
                (sret::FUNCT3, sret::IMM) => Ok(InstructionDecoded::SRet),

                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Csr I-Type instruction")
            }
        }
        // TODO: SFenceVMA
        _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown I-Type instruction"),
    }
}

pub fn decode_stype(inst: InstructionSize) -> Result<InstructionDecoded> {
    let sinst = stype::SType::new(inst);
    match sinst.opcode() {
        STORE_MATCH => match sinst.funct3() {
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
        FSTORE_MATCH => match sinst.funct3() {
            fsw::FUNCT3 => Ok(InstructionDecoded::Fsw {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            fsd::FUNCT3 => Ok(InstructionDecoded::Fsd {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown S-Type instruction"),
        }
        _ => return Err(DecodeError::UnknownInstructionFormat).context("Unknown S-Type instruction"),
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
        FLOATING_POINT_MATCH | ATOMIC_MATCH | ARITMETIC_REGISTER_MATCH => InstructionFormat::RType,
        FSTORE_MATCH | STORE_MATCH => InstructionFormat::SType,
        BRANCH_MATCH => InstructionFormat::BType,
        JAL_MATCH => InstructionFormat::JType,
        FLOAD_MATCH | ARITMETIC_IMMEDIATE_MATCH | FENCE_MATCH | LOAD_MATCH | CSR_MATCH | JALR_MATCH => {
            InstructionFormat::IType
        }
        LUI_MATCH | AUIPC_MATCH => InstructionFormat::UType,
        _ => Err(DecodeError::UnknownInstructionFormat)
            .context(format!("Failed to decode inst {inst}"))?,
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
    Err(DecodeError::UnknownInstructionFormat)
        .context(format!("Compressed instructions are not supported yet"))
}

macro_rules! decode_test {
    ($inst:ident, $value:expr, $expected:expr) => {
        paste! {
            #[test]
            fn [<test_decode_ $inst>]() {
                let inst = try_decode($value).expect("Failed to decode inst");
                assert_eq!(inst, $expected);
            }
        }
    };
}

decode_test!(
    amoswap_w,
    0xCF4A7AF, /* amoswap.w x15, x15, (x9) */
    InstructionDecoded::AmoswapW {
        rd: 15,
        rs1: 9,
        rs2: 15,
        rl: false,
        aq: true,
    }
);

decode_test!(
    fcvt_s_w,
    0xd00777d3, /* fcvt.s.w fa5, a4 */
    InstructionDecoded::FcvtSW { rd: 15, rs1: 14 }
);

decode_test!(
    fcvt_w_s,
    0xc00777d3, /* fcvt.w.s a5, fa4 */
    InstructionDecoded::FcvtWUS { rd: 15, rs1: 14 }
);

// TODO: add more tests!
