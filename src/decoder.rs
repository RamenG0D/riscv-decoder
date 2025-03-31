use crate::{decoded_inst::Instruction, errors::DecodeError, instructions::*};
use bit_ops::bitops_u32 as bit_ops;
use cached::proc_macro::cached;
use anyhow::{Context, Result};
use ::bit_ops::BitOps;
use paste::paste;

pub const OPCODE_MASK: InstructionSize = bit_ops::create_mask(7);
// basically the opcode mask but for a compressed instruction (a compresed inst's opcode is the first 2 bits)
pub const COMPRESSED_MASK: InstructionSize = bit_ops::create_mask(2);

pub fn decode_rtype(inst: InstructionSize) -> Result<Instruction> {
    let inst = rtype::RType::new(inst);
    match inst.opcode() {
        ARITMETIC_REGISTER_FMT => {
            match (inst.funct3(), inst.funct7()) {
                (add::FUNCT3, add::FUNCT7) => Ok(Instruction::Add {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (sub::FUNCT3, sub::FUNCT7) => Ok(Instruction::Sub {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (sll::FUNCT3, sll::FUNCT7) => Ok(Instruction::Sll {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (slt::FUNCT3, slt::FUNCT7) => Ok(Instruction::Slt {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (sltu::FUNCT3, sltu::FUNCT7) => Ok(Instruction::Sltu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (xor::FUNCT3, xor::FUNCT7) => Ok(Instruction::Xor {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (srl::FUNCT3, srl::FUNCT7) => Ok(Instruction::Srl {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (sra::FUNCT3, sra::FUNCT7) => Ok(Instruction::Sra {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (or::FUNCT3, or::FUNCT7) => Ok(Instruction::Or {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (and::FUNCT3, and::FUNCT7) => Ok(Instruction::And {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),

                // M extension
                (mul::FUNCT3, mul::FUNCT7) => Ok(Instruction::Mul {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (mulh::FUNCT3, mulh::FUNCT7) => Ok(Instruction::Mulh {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (mulu::FUNCT3, mulu::FUNCT7) => Ok(Instruction::Mulu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (mulsu::FUNCT3, mulsu::FUNCT7) => Ok(Instruction::Mulsu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (div::FUNCT3, div::FUNCT7) => Ok(Instruction::Div {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (divu::FUNCT3, divu::FUNCT7) => Ok(Instruction::Divu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (rem::FUNCT3, rem::FUNCT7) => Ok(Instruction::Rem {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),
                (remu::FUNCT3, remu::FUNCT7) => Ok(Instruction::Remu {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                }),

                _ => Err(DecodeError::InvalidInstruction(inst.0))
                    .context("Unknown Arithmetic Register instruction"),
            }
        }
        ATOMIC_FMT => {
            let funct5 = inst.funct7().get_bits(5, 2);
            let rl = inst.funct7().is_set(0);
            let aq = inst.funct7().is_set(1);
            match (inst.funct3(), funct5) {
                (amoswap_w::FUNCT3, amoswap_w::FUNCT5) => Ok(Instruction::AmoswapW {
                    rd: inst.rd(),
                    rs1: inst.rs1(),
                    rs2: inst.rs2(),
                    rl,
                    aq,
                }),
                _ => Err(DecodeError::InvalidInstruction(inst.0))
                    .context("Unknown Atomic instruction"),
            }
        }
        FLOATING_POINT_FMT => {
            let funct5 = inst.funct7().get_bits(5, 2);
            let fmt = inst.funct7().get_bits(2, 0) as u8;
            const SINGLE_PRECISION_FMT: u8 = 0;
            const DOUBLE_PRECISION_FMT: u8 = 1;
            const QUAD_PRECISION_FMT: u8 = 3;
            match fmt {
                SINGLE_PRECISION_FMT => decode_single_precision(&inst, inst.funct3(), funct5),
                DOUBLE_PRECISION_FMT => decode_double_precision(&inst, inst.funct3(), funct5),
                QUAD_PRECISION_FMT => decode_quad_precision(&inst, inst.funct3(), funct5),
                _ => Err(DecodeError::InvalidInstruction(inst.0))
                    .context("Unknown Floating Point instruction"),
            }
        }
        FMADDD_FMT => {
            let rs3 = inst.funct7().get_bits(5, 2);
            Ok(Instruction::FmaddD {
                rd: inst.rd(),
                rs1: inst.rs1(),
                rs2: inst.rs2(),
                rs3,
            })
        }
        FMSUBD_FMT => {
            let rs3 = inst.funct7().get_bits(5, 2);
            Ok(Instruction::FmsubD {
                rd: inst.rd(),
                rs1: inst.rs1(),
                rs2: inst.rs2(),
                rs3,
            })
        }
        FNMADDD_FMT => {
            let rs3 = inst.funct7().get_bits(5, 2);
            Ok(Instruction::FnmaddD {
                rd: inst.rd(),
                rs1: inst.rs1(),
                rs2: inst.rs2(),
                rs3,
            })
        }
        FNMSUBD_FMT => {
            let rs3 = inst.funct7().get_bits(5, 2);
            Ok(Instruction::FnmsubD {
                rd: inst.rd(),
                rs1: inst.rs1(),
                rs2: inst.rs2(),
                rs3,
            })
        }
        _ => Err(DecodeError::InvalidInstruction(inst.0)).context("Unknown R-Type instruction"),
    }
}

fn decode_single_precision(
    inst: &rtype::RType,
    funct3: InstructionSize,
    funct5: InstructionSize,
) -> Result<Instruction> {
    match (funct3, funct5) {
        (_, fadd_s::FUNCT5) => Ok(Instruction::FaddS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fsub_s::FUNCT5) => Ok(Instruction::FsubS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fmul_s::FUNCT5) => Ok(Instruction::FmulS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fdiv_s::FUNCT5) => Ok(Instruction::FdivS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fsqrt_s::FUNCT5) => Ok(Instruction::FsqrtS {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (fsgnj_s::FUNCT3, fsgnj_s::FUNCT5) => Ok(Instruction::FsgnjS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjn_s::FUNCT3, fsgnjn_s::FUNCT5) => Ok(Instruction::FsgnjnS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjx_s::FUNCT3, fsgnjx_s::FUNCT5) => Ok(Instruction::FsgnjxS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmin_s::FUNCT3, fmin_s::FUNCT5) => Ok(Instruction::FminS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmax_s::FUNCT3, fmax_s::FUNCT5) => Ok(Instruction::FmaxS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fcvt_w_s::FUNCT5) => match inst.rs2() {
            fcvt_w_s::RS2 => Ok(Instruction::FcvtWS {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            fcvt_wu_s::RS2 => Ok(Instruction::FcvtWUS {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            _ => {
                Err(DecodeError::InvalidInstruction(inst.0)).context("Unknown Fcvt W S instruction")
            }
        },
        (feq_s::FUNCT3, feq_s::FUNCT5) => Ok(Instruction::FeqS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (flt_s::FUNCT3, flt_s::FUNCT5) => Ok(Instruction::FltS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fle_s::FUNCT5) => Ok(Instruction::FleS {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fclass_s::FUNCT3, fclass_s::FUNCT5) => Ok(Instruction::FClassS {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (_, fcvt_s_w::FUNCT5) => Ok(Instruction::FcvtSW {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (_, fcvt_s_wu::FUNCT5) => Ok(Instruction::FcvtSWU {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (_, fmv_x_w::FUNCT5) => Ok(Instruction::FmvXW {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (_, fmv_w_x::FUNCT5) => Ok(Instruction::FmvWX {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (_, fcvt_s_d::FUNCT5) => Ok(Instruction::FcvtSD {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        _ => Err(DecodeError::InvalidInstruction(inst.0))
            .context("Unknown Single Precision Floating Point instruction"),
    }
}
fn decode_double_precision(
    inst: &rtype::RType,
    funct3: InstructionSize,
    funct5: InstructionSize,
) -> Result<Instruction> {
    match (funct3, funct5) {
        (_, fcvt_d_w::FUNCT5) => match inst.rs2() {
            fcvt_d_w::RS2 => Ok(Instruction::FcvtDW {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            fcvt_d_wu::RS2 => Ok(Instruction::FcvtDWU {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            _ => {
                Err(DecodeError::InvalidInstruction(inst.0)).context("Unknown Fcvt D W instruction")
            }
        },
        (_, fadd_d::FUNCT5) => Ok(Instruction::FaddD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fsub_d::FUNCT5) => Ok(Instruction::FsubD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fmul_d::FUNCT5) => Ok(Instruction::FmulD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fdiv_d::FUNCT5) => Ok(Instruction::FdivD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnj_d::FUNCT3, fsgnj_d::FUNCT5) => Ok(Instruction::FsgnjD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjn_d::FUNCT3, fsgnjn_d::FUNCT5) => Ok(Instruction::FsgnjnD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fsgnjx_d::FUNCT3, fsgnjx_d::FUNCT5) => Ok(Instruction::FsgnjxD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmin_d::FUNCT3, fmin_d::FUNCT5) => Ok(Instruction::FminD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fmax_d::FUNCT3, fmax_d::FUNCT5) => Ok(Instruction::FmaxD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (_, fcvt_d_s::FUNCT5) => Ok(Instruction::FcvtDS {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (feq_d::FUNCT3, feq_d::FUNCT5) => Ok(Instruction::FeqD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (flt_d::FUNCT3, flt_d::FUNCT5) => Ok(Instruction::FltD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fle_d::FUNCT3, fle_d::FUNCT5) => Ok(Instruction::FleD {
            rd: inst.rd(),
            rs1: inst.rs1(),
            rs2: inst.rs2(),
        }),
        (fclass_d::FUNCT3, fclass_d::FUNCT5) => Ok(Instruction::FClassD {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        (_, fcvt_w_d::FUNCT5) => match inst.rs2() {
            fcvt_w_d::RS2 => Ok(Instruction::FcvtWD {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            fcvt_wu_d::RS2 => Ok(Instruction::FcvtWUD {
                rd: inst.rd(),
                rs1: inst.rs1(),
            }),
            _ => {
                Err(DecodeError::InvalidInstruction(inst.0)).context("Unknown Fcvt W D instruction")
            }
        },
        (_, fsqrt_d::FUNCT5) => Ok(Instruction::FsqrtD {
            rd: inst.rd(),
            rs1: inst.rs1(),
        }),
        // (fmv_x_d::FUNCT3, fmv_x_d::FUNCT5) => Ok(InstructionDecoded::FmvXD {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        // (fmv_d_x::FUNCT3, fmv_d_x::FUNCT5) => Ok(InstructionDecoded::FmvDX {
        //     rd: inst.rd(),
        //     rs1: inst.rs1(),
        // }),
        _ => Err(DecodeError::InvalidInstruction(inst.0))
            .context("Unknown Double Precision Floating Point instruction"),
    }
}

fn decode_quad_precision(
    inst: &rtype::RType,
    _funct3: InstructionSize,
    _funct5: InstructionSize,
) -> Result<Instruction> {
    Err(DecodeError::InvalidInstruction(inst.0))
        .context("Quad Precision Floating Point instructions are not supported yet")
}

pub fn decode_itype(inst: InstructionSize) -> Result<Instruction> {
    let iinst = itype::IType::new(inst);
    match iinst.opcode() {
        ARITMETIC_IMMEDIATE_FMT => {
            let imm = iinst.imm().get_bits(5, 0); // remove bits [11:5]
            let f5 = iinst.imm().get_bits(6, 5); // get bits [11:5] for the funct5
            match (iinst.funct3(), f5) {
                (addi::FUNCT3, _) => Ok(Instruction::Addi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (slti::FUNCT3, _) => Ok(Instruction::Slti {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (sltiu::FUNCT3, _) => Ok(Instruction::Sltiu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (xori::FUNCT3, _) => Ok(Instruction::Xori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (ori::FUNCT3, _) => Ok(Instruction::Ori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (andi::FUNCT3, _) => Ok(Instruction::Andi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm(),
                }),
                (slli::FUNCT3, slli::IMM) => Ok(Instruction::Slli {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm,
                }),
                (srli::FUNCT3, srli::IMM) => Ok(Instruction::Srli {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm,
                }),
                (srai::FUNCT3, srai::IMM) => Ok(Instruction::Srai {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm,
                }),
                _ => Err(DecodeError::InvalidInstruction(inst))
                    .context("Unknown Arithmetic immediate I-Type instruction"),
            }
        }
        LOAD_FMT => match iinst.funct3() {
            lb::FUNCT3 => Ok(Instruction::Lb {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lh::FUNCT3 => Ok(Instruction::Lh {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lw::FUNCT3 => Ok(Instruction::Lw {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lbu::FUNCT3 => Ok(Instruction::Lbu {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            lhu::FUNCT3 => Ok(Instruction::Lhu {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            _ => Err(DecodeError::InvalidInstruction(inst))
                .context("Unknown Load I-Type instruction"),
        },
        FLOAD_FMT => match iinst.funct3() {
            flw::FUNCT3 => Ok(Instruction::Flw {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            fld::FUNCT3 => Ok(Instruction::Fld {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: iinst.imm(),
            }),
            _ => Err(DecodeError::InvalidInstruction(inst))
                .context("Unknown Float Load I-Type instruction"),
        },
        JALR_FMT if iinst.funct3() == jalr::FUNCT3 => Ok(Instruction::Jalr {
            rd: iinst.rd(),
            rs1: iinst.rs1(),
            imm: iinst.imm(),
        }),
        FENCE_FMT => {
            let pred = iinst.imm().get_bits(4, 0);
            let succ = iinst.imm().get_bits(4, 4);
            match iinst.funct3() {
                fence::FUNCT3 => Ok(Instruction::Fence { pred, succ }),
                fence_i::FUNCT3 => Ok(Instruction::FenceI { pred, succ }),
                _ => Err(DecodeError::InvalidInstruction(inst))
                    .context("Unknown Fence I-Type instruction"),
            }
        }
        CSR_FMT => {
            match (iinst.funct3(), iinst.imm()) {
                (csrrw::FUNCT3, _) => Ok(Instruction::CsrRw {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrs::FUNCT3, _) => Ok(Instruction::CsrRs {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrc::FUNCT3, _) => Ok(Instruction::CsrRc {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrwi::FUNCT3, _) => Ok(Instruction::CsrRwi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrsi::FUNCT3, _) => Ok(Instruction::CsrRsi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                (csrrci::FUNCT3, _) => Ok(Instruction::CsrRci {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.uimm(),
                }),
                // e-insts (ebreak, ecall)
                (sfencevma::FUNCT3, sfencevma::IMM) => Ok(Instruction::SFenceVma),
                (ebreak::FUNCT3, ebreak::IMM) => Ok(Instruction::EBreak),
                (ecall::FUNCT3, ecall::IMM) => Ok(Instruction::ECall),
                (mret::FUNCT3, mret::IMM) => Ok(Instruction::MRet),
                (sret::FUNCT3, sret::IMM) => Ok(Instruction::SRet),

                _ => Err(DecodeError::InvalidInstruction(inst))
                    .context("Unknown Csr I-Type instruction"),
            }
        }
        _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown I-Type instruction"),
    }
}

pub fn decode_stype(inst: InstructionSize) -> Result<Instruction> {
    let sinst = stype::SType::new(inst);
    match sinst.opcode() {
        STORE_FMT => match sinst.funct3() {
            sb::FUNCT3 => Ok(Instruction::Sb {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            sh::FUNCT3 => Ok(Instruction::Sh {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            sw::FUNCT3 => Ok(Instruction::Sw {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown S-Type instruction"),
        },
        FSTORE_FMT => match sinst.funct3() {
            fsw::FUNCT3 => Ok(Instruction::Fsw {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            fsd::FUNCT3 => Ok(Instruction::Fsd {
                rs1: sinst.rs1(),
                rs2: sinst.rs2(),
                imm: sinst.imm(),
            }),
            _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown S-Type instruction"),
        },
        _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown S-Type instruction"),
    }
}

pub fn decode_utype(inst: InstructionSize) -> Result<Instruction> {
    let uinst = utype::UType::new(inst);
    match uinst.opcode() {
        LUI_FMT => Ok(Instruction::Lui {
            rd: uinst.rd(),
            imm: uinst.imm(),
        }),
        AUIPC_FMT => Ok(Instruction::AuiPc {
            rd: uinst.rd(),
            imm: uinst.imm(),
        }),
        _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown U-Type instruction"),
    }
}

pub fn decode_btype(inst: InstructionSize) -> Result<Instruction> {
    let binst = btype::BType::new(inst);
    match binst.funct3() {
        beq::FUNCT3 => Ok(Instruction::Beq {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bne::FUNCT3 => Ok(Instruction::Bne {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        blt::FUNCT3 => Ok(Instruction::Blt {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bge::FUNCT3 => Ok(Instruction::Bge {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bltu::FUNCT3 => Ok(Instruction::Bltu {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        bgeu::FUNCT3 => Ok(Instruction::Bgeu {
            rs1: binst.rs1(),
            rs2: binst.rs2(),
            imm: binst.imm(),
        }),
        _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown B-Type instruction"),
    }
}

pub fn decode_jtype(inst: InstructionSize) -> Result<Instruction> {
    let jinst = jtype::JType::new(inst);
    match jinst.opcode() {
        JAL_FMT => Ok(Instruction::Jal {
            rd: jinst.rd(),
            imm: jinst.imm(),
        }),
        _ => Err(DecodeError::InvalidInstruction(inst)).context("Unknown J-Type instruction"),
    }
}

#[cached(result = true)]
pub fn try_decode(inst: InstructionSize) -> Result<Instruction> {
    // if its a compressed inst then dont bother with regular decoding, instead decode it as compressed and return the result
    match inst & COMPRESSED_MASK {
        // its a compressed instruction
        0 | 1 | 2 => return try_decode_compressed(inst),
        // otherwise just continue with regular decoding
        _ => (),
    }

    let inst = match InstructionFormat::try_from(inst)? {
        InstructionFormat::RType => decode_rtype(inst)?,
        InstructionFormat::IType => decode_itype(inst)?,
        InstructionFormat::SType => decode_stype(inst)?,
        InstructionFormat::UType => decode_utype(inst)?,
        InstructionFormat::BType => decode_btype(inst)?,
        InstructionFormat::JType => decode_jtype(inst)?,
    };

    Ok(inst)
}

pub fn try_decode_compressed(inst: InstructionSize) -> Result<Instruction> {
    Err(DecodeError::InvalidInstruction(inst))
        .context("Compressed instructions are not supported yet")
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
    Instruction::AmoswapW {
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
    Instruction::FcvtSW { rd: 15, rs1: 14 }
);

decode_test!(
    fcvt_w_s,
    0xc00777d3, /* fcvt.w.s a5, fa4 */
    Instruction::FcvtWS { rd: 15, rs1: 14 }
);

decode_test!(
    fcvt_d_w,
    0xD20507D3, /* fcvt.d.w fa5, a0, rne */
    Instruction::FcvtDW { rd: 15, rs1: 10 }
);

decode_test!(
    fcvt_w_d,
    0xc2079553, /* fcvt.w.d a0, fa5, rtz */
    Instruction::FcvtWD { rd: 10, rs1: 15 }
);

decode_test!(
    fmadd_d,
    0x0af5f7c3, /* fmadd.d fa5, fa1, fa5, ft1 */
    Instruction::FmaddD {
        rd: 15,
        rs1: 11,
        rs2: 15,
        rs3: 1
    }
);

decode_test!(
    fmsub_d,
    0x1207f047, /* fmsub.d ft0, fa5, ft0, ft2 */
    Instruction::FmsubD {
        rd: 0,
        rs1: 15,
        rs2: 0,
        rs3: 2
    }
);

decode_test!(
    fmv_w_x,
    0xf0000053, /* fmv.w.x f0, x0 */
    Instruction::FmvWX { rd: 0, rs1: 0 }
);

decode_test!(
    fcvt_d_s,
    0x42078753, /* fcvt.d.s f14, f15, rne */
    Instruction::FcvtDS { rd: 14, rs1: 15 }
);

decode_test!(
    fcvt_s_d,
    0x4017F7D3, /* fcvt.s.d f15, f15 */
    Instruction::FcvtSD { rd: 15, rs1: 15 }
);

decode_test!(
    fnmsub_d,
    0x5A07F5CB, /* fnmsub.d fa1, fa5, ft0, fa1 */
    Instruction::FnmsubD {
        rd: 11,
        rs1: 15,
        rs2: 0,
        rs3: 11
    }
);

decode_test!(
    srai,
    0x4010d093, /* srai ra, ra, 0x1 */
    Instruction::Srai {
        rd: 1,
        rs1: 1,
        imm: 1
    }
);

// TODO: add more tests!
