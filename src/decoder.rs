use crate::{decoded_inst::InstructionDecoded, error::DecodeError, instructions::*};
use compressed::is_compressed;

pub fn decode(inst: InstructionSize) -> Result<(InstructionDecoded, bool), DecodeError> {
    if is_compressed(inst) {
        try_decode_compressed(inst).map(|inst| (inst, true))
    } else {
        try_decode(inst).map(|inst| (inst, false))
    }
}

pub fn try_decode_compressed(_inst: InstructionSize) -> Result<InstructionDecoded, DecodeError> {
    unimplemented!("Compressed instructions are not yet implemented")
}

pub fn try_decode(inst: InstructionSize) -> Result<InstructionDecoded, DecodeError> {
    const OPCODE_MASK: InstructionSize = 0b1111111;

    let fmt = match inst & OPCODE_MASK {
        ARITMETIC_REGISTER_MATCH =>  InstructionFormat::RType,
        STORE_MATCH =>               InstructionFormat::SType,
        BRANCH_MATCH =>              InstructionFormat::BType,
        JAL_MATCH =>                 InstructionFormat::JType,
        ARITMETIC_IMMEDIATE_MATCH |
        FENCE_MATCH |
        LOAD_MATCH |
        CSR_MATCH |
        JALR_MATCH =>                InstructionFormat::IType,
        LUI_MATCH |
        AUIPC_MATCH =>               InstructionFormat::UType,
        v => Err(DecodeError::UnknownInstructionFormat(format!("Unknown InstructionFormat for instruction: {:#X}({:#X})", inst, v)))?,
    };

    let inst = match fmt {
        InstructionFormat::RType => {
            let rinst = rtype::RType::new(inst);
            match (rinst.funct3(), rinst.funct7()) {
                (add::FUNCT3, add::FUNCT7) => InstructionDecoded::Add {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (sub::FUNCT3, sub::FUNCT7) => InstructionDecoded::Sub {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (sll::FUNCT3, sll::FUNCT7) => InstructionDecoded::Sll {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (slt::FUNCT3, slt::FUNCT7) => InstructionDecoded::Slt {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (sltu::FUNCT3, sltu::FUNCT7) => InstructionDecoded::Sltu {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (xor::FUNCT3, xor::FUNCT7) => InstructionDecoded::Xor {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (srl::FUNCT3, srl::FUNCT7) => InstructionDecoded::Srl {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (sra::FUNCT3, sra::FUNCT7) => InstructionDecoded::Sra {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (or::FUNCT3, or::FUNCT7) => InstructionDecoded::Or {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (and::FUNCT3, and::FUNCT7) => InstructionDecoded::And {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown R-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::IType => {
            let iinst = itype::IType::new(inst);
            match (iinst.opcode(), iinst.funct3(), iinst.imm1() as InstructionSize) {
                (LOAD_MATCH, lb::FUNCT3, _) => InstructionDecoded::Lb {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (LOAD_MATCH, lh::FUNCT3, _) => InstructionDecoded::Lh {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (LOAD_MATCH, lw::FUNCT3, _) => InstructionDecoded::Lw {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (LOAD_MATCH, lbu::FUNCT3, _) => InstructionDecoded::Lbu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (LOAD_MATCH, lhu::FUNCT3, _) => InstructionDecoded::Lhu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, addi::FUNCT3, _) => InstructionDecoded::Addi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, slli::FUNCT3, _) => InstructionDecoded::Slli {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, slti::FUNCT3, _) => InstructionDecoded::Slti {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, sltiu::FUNCT3, _) => InstructionDecoded::Sltiu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, xori::FUNCT3, _) => InstructionDecoded::Xori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, srli::FUNCT3, 0) => InstructionDecoded::Srli {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, srai::FUNCT3, 32) => InstructionDecoded::Srai {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, ori::FUNCT3, _) => InstructionDecoded::Ori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (ARITMETIC_IMMEDIATE_MATCH, andi::FUNCT3, _) => InstructionDecoded::Andi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (JALR_MATCH, 0 /* f3 must be zero */, _) => InstructionDecoded::Jalr {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },

                (CSR_MATCH, csrrw::FUNCT3, _) => InstructionDecoded::CsrRw {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (CSR_MATCH, csrrs::FUNCT3, _) => InstructionDecoded::CsrRs {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (CSR_MATCH, csrrc::FUNCT3, _) => InstructionDecoded::CsrRc {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (CSR_MATCH, csrrwi::FUNCT3, _) => InstructionDecoded::CsrRwi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (CSR_MATCH, csrrsi::FUNCT3, _) => InstructionDecoded::CsrRsi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (CSR_MATCH, csrrci::FUNCT3, _) => InstructionDecoded::CsrRci {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },

                (CSR_MATCH, ecall::FUNCT3, ecall::FUNCT7) => InstructionDecoded::ECall,
                (CSR_MATCH, ebreak::FUNCT3, ebreak::FUNCT7) => InstructionDecoded::EBreak,
                (CSR_MATCH, sret::FUNCT3, sret::FUNCT7) => InstructionDecoded::SRet,
                (CSR_MATCH, mret::FUNCT3, mret::FUNCT7) => InstructionDecoded::MRet,
                (CSR_MATCH, sfence_vma::FUNCT3, sfence_vma::FUNCT7) => InstructionDecoded::SFenceVma,
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown I-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::SType => {
            let sinst = stype::SType::new(inst);
            match sinst.funct3() {
                sb::FUNCT3 => InstructionDecoded::Sb {
                    rs1: sinst.rs1(),
                    rs2: sinst.rs2(),
                    imm: sinst.imm(),
                },
                sh::FUNCT3 => InstructionDecoded::Sh {
                    rs1: sinst.rs1(),
                    rs2: sinst.rs2(),
                    imm: sinst.imm(),
                },
                sw::FUNCT3 => InstructionDecoded::Sw {
                    rs1: sinst.rs1(),
                    rs2: sinst.rs2(),
                    imm: sinst.imm(),
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown S-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::UType => {
            let uinst = utype::UType::new(inst);
            match uinst.opcode() {
                lui::OPCODE => InstructionDecoded::Lui {
                    rd: uinst.rd(),
                    imm: uinst.imm1() as InstructionSize,
                },
                auipc::OPCODE => InstructionDecoded::AuiPc {
                    rd: uinst.rd(),
                    imm: uinst.imm1() as InstructionSize,
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown U-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::BType => {
            let binst = btype::BType::new(inst);
            match binst.funct3() {
                beq::FUNCT3 => InstructionDecoded::Beq {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                bne::FUNCT3 => InstructionDecoded::Bne {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                blt::FUNCT3 => InstructionDecoded::Blt {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                bge::FUNCT3 => InstructionDecoded::Bge {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                bltu::FUNCT3 => InstructionDecoded::Bltu {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                bgeu::FUNCT3 => InstructionDecoded::Bgeu {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown B-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::JType => {
            let jinst = jtype::JType::new(inst);
            match jinst.opcode() {
                jal::OPCODE => InstructionDecoded::Jal {
                    rd: jinst.rd(),
                    imm: jinst.imm() as InstructionSize,
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown J-Type instruction: {:#X}", inst)))?
            }
        }
    };

    Ok(inst)
}
