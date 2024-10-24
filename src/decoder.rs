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
            let fmt = get_bits(inst.funct7(), 2, 0);
            assert!(fmt == 0, "the fmt of an inst cannot be non 0 because we only support single precision floating point instructions currently!");
            match (inst.funct3(), funct5) {
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
                _ => Err(DecodeError::UnknownInstructionFormat).context("Unknown Floating Point instruction"),
            }
        }

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
        imm @ (ARITMETIC_IMMEDIATE_MATCH, slli::FUNCT3, _) if (imm.2 >> 5) == slli::IMM => {
            Ok(InstructionDecoded::Slli {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: get_bits(imm.2, 5, 0),
            })
        }
        imm @ (ARITMETIC_IMMEDIATE_MATCH, srli::FUNCT3, _) if (imm.2 >> 5) == srli::IMM => {
            Ok(InstructionDecoded::Srli {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: get_bits(imm.2, 5, 0),
            })
        }
        imm @ (ARITMETIC_IMMEDIATE_MATCH, srai::FUNCT3, _) if (imm.2 >> 5) == srai::IMM => {
            Ok(InstructionDecoded::Srai {
                rd: iinst.rd(),
                rs1: iinst.rs1(),
                imm: get_bits(imm.2, 5, 0),
            })
        }
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
            let pred = get_bits(iinst.imm(), 4, 0);
            let succ = get_bits(iinst.imm() >> 4, 4, 0);
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
        (CSR_MATCH, sfencevma::FUNCT3, sfencevma::IMM) => Ok(InstructionDecoded::SFenceVma),
        (CSR_MATCH, ebreak::FUNCT3, ebreak::IMM) => Ok(InstructionDecoded::EBreak),
        (CSR_MATCH, ecall::FUNCT3, ecall::IMM) => Ok(InstructionDecoded::ECall),
        (CSR_MATCH, mret::FUNCT3, mret::IMM) => Ok(InstructionDecoded::MRet),
        (CSR_MATCH, sret::FUNCT3, sret::IMM) => Ok(InstructionDecoded::SRet),
        // TODO: SFenceVMA
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
        FLOATING_POINT_MATCH | ATOMIC_MATCH | ARITMETIC_REGISTER_MATCH => InstructionFormat::RType,
        STORE_MATCH => InstructionFormat::SType,
        BRANCH_MATCH => InstructionFormat::BType,
        JAL_MATCH => InstructionFormat::JType,
        ARITMETIC_IMMEDIATE_MATCH | FENCE_MATCH | LOAD_MATCH | CSR_MATCH | JALR_MATCH => {
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
