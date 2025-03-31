use std::fmt::Display;

use crate::instructions::InstructionSize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Lb {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lh {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lw {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lbu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lhu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lwu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Addi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Slli {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Slti {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Sltiu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Xori {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Srli {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Srai {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Ori {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Andi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    AuiPc {
        rd: InstructionSize,
        imm: InstructionSize,
    },
    Sb {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Sh {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Sw {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Add {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sub {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sll {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Slt {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sltu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Xor {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Srl {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sra {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Or {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    And {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Lui {
        rd: InstructionSize,
        imm: InstructionSize,
    },
    Beq {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bne {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Blt {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bge {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bltu {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bgeu {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Jalr {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Jal {
        rd: InstructionSize,
        imm: InstructionSize,
    },

    ECall,
    EBreak,
    SRet,
    MRet,
    SFenceVma,

    CsrRw {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRs {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRc {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRwi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRsi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRci {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },

    Fence {
        // rd: InstructionSize,
        // rs1: InstructionSize,
        // fm: InstructionSize,
        pred: InstructionSize,
        succ: InstructionSize,
    },
    FenceI {
        // rd: InstructionSize,
        // rs1: InstructionSize,
        // fm: InstructionSize,
        pred: InstructionSize,
        succ: InstructionSize,
    },

    // F Extension (floats)
    Flw {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Fsw {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    FmaddS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FmsubS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FnmaddS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FnmsubS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FaddS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsubS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FmulS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FdivS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsqrtS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FsgnjS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsgnjnS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsgnjxS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FminS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FmaxS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FcvtSW {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtSWU {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtWS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtWUS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtWD {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtWUD {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtDW {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtDWU {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FmvXW {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FmvWX {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FeqS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FltS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FleS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FClassS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },

    // D Extension
    FmaddD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
	FmsubD {
		rd: InstructionSize,
		rs1: InstructionSize,
		rs2: InstructionSize,
		rs3: InstructionSize,
	},

	FnmaddD {
		rd: InstructionSize,
		rs1: InstructionSize,
		rs2: InstructionSize,
		rs3: InstructionSize,
	},
	FnmsubD {
		rd: InstructionSize,
		rs1: InstructionSize,
		rs2: InstructionSize,
		rs3: InstructionSize,
	},

    Fld {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Fsd {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    FcvtSD {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtDS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FaddD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsubD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FmulD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FdivD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsqrtD {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FsgnjD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsgnjnD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsgnjxD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FminD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FmaxD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FeqD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FltD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FleD {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FClassD {
        rd: InstructionSize,
        rs1: InstructionSize,
    },

    // M Extension
    Mul {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Mulh {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Mulsu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Mulu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Div {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Divu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Rem {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Remu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },

    // A Extension
    LrW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    ScW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmoswapW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmoaddW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmoandW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmoorW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmoxorW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmomaxW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },
    AmominW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: bool,
        aq: bool,
    },

    // Compressed Instructions
    CAddi4Spn {
        rd: InstructionSize,
        nzuimm: InstructionSize,
    },
    CNop,
    CSlli {
        rd: InstructionSize,
        rs1: InstructionSize,
        shamt: InstructionSize,
    },
}

// generates comptime map for large amount of csr mapping their names to their values
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

macro_rules! print_csr {
    ($f:expr, $name:expr, $name_exp:expr, $rd:ident, $rs1:ident, $imm:ident) => {
        if *$rd == 0 || *$rd == *$rs1 {
            write!(
                $f,
                "{} {}, {}",
                $name,
                CSRS.get($imm)
                    .map(|v| *v)
                    .unwrap_or(format!("{}", $imm).as_str()),
                REG_NAMES[*$rs1 as usize]
            )
        } else {
            write!(
                $f,
                "{} {}, {}, {}",
                $name_exp,
                REG_NAMES[*$rd as usize],
                CSRS.get($imm)
                    .map(|v| *v)
                    .unwrap_or(format!("{}", $imm).as_str()),
                REG_NAMES[*$rs1 as usize]
            )
        }
    };
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const REG_NAMES: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];

        match self {
            Instruction::Lb { rd, rs1, imm } => {
                write!(
                    f,
                    "lb {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Lh { rd, rs1, imm } => {
                write!(
                    f,
                    "lh {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Lw { rd, rs1, imm } => {
                write!(
                    f,
                    "lw {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Lbu { rd, rs1, imm } => {
                write!(
                    f,
                    "lbu {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Lhu { rd, rs1, imm } => {
                write!(
                    f,
                    "lhu {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Lwu { rd, rs1, imm } => {
                write!(
                    f,
                    "lwu {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Addi { rd, rs1, imm } => {
                write!(
                    f,
                    "addi {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Slli { rd, rs1, imm } => {
                write!(
                    f,
                    "slli {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Slti { rd, rs1, imm } => {
                write!(
                    f,
                    "slti {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                write!(
                    f,
                    "sltiu {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Xori { rd, rs1, imm } => {
                write!(
                    f,
                    "xori {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Srli { rd, rs1, imm } => {
                write!(
                    f,
                    "srli {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Srai { rd, rs1, imm } => {
                write!(
                    f,
                    "srai {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Ori { rd, rs1, imm } => {
                write!(
                    f,
                    "ori {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::Andi { rd, rs1, imm } => {
                write!(
                    f,
                    "andi {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32
                )
            }
            Instruction::AuiPc { rd, imm } => {
                write!(f, "auipc {}, {}", REG_NAMES[*rd as usize], *imm as i32)
            }
            Instruction::Sb { rs1, rs2, imm } => {
                write!(
                    f,
                    "sb {}, {}({})",
                    REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Sh { rs1, rs2, imm } => {
                write!(
                    f,
                    "sh {}, {}({})",
                    REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Sw { rs1, rs2, imm } => {
                write!(
                    f,
                    "sw {}, {}({})",
                    REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Add { rd, rs1, rs2 } => {
                write!(
                    f,
                    "add {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                write!(
                    f,
                    "sub {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                write!(
                    f,
                    "sll {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                write!(
                    f,
                    "slt {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                write!(
                    f,
                    "sltu {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                write!(
                    f,
                    "xor {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                write!(
                    f,
                    "srl {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                write!(
                    f,
                    "sra {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Or { rd, rs1, rs2 } => {
                write!(
                    f,
                    "or {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::And { rd, rs1, rs2 } => {
                write!(
                    f,
                    "and {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Lui { rd, imm } => {
                write!(f, "lui {}, {:#X}", REG_NAMES[*rd as usize], *imm)
            }
            Instruction::Beq { rs1, rs2, imm } => {
                write!(
                    f,
                    "beq {}, {}, {}",
                    REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32
                )
            }
            Instruction::Bne { rs1, rs2, imm } => {
                write!(
                    f,
                    "bne {}, {}, {}",
                    REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32
                )
            }
            Instruction::Blt { rs1, rs2, imm } => {
                write!(
                    f,
                    "blt {}, {}, {}",
                    REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32
                )
            }
            Instruction::Bge { rs1, rs2, imm } => {
                write!(
                    f,
                    "bge {}, {}, {}",
                    REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32
                )
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                write!(
                    f,
                    "bltu {}, {}, {}",
                    REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32
                )
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                write!(
                    f,
                    "bgeu {}, {}, {}",
                    REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32
                )
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let args = match (*imm as i32 == 0, rd == rs1) {
                    (true, true) => format!("{}", REG_NAMES[*rd as usize]),
                    (true, false) => {
                        format!("{}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
                    }
                    (false, true) => format!("{}({})", *imm as i32, REG_NAMES[*rd as usize]),
                    (false, false) => format!(
                        "{}, {}({})",
                        REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                    ),
                };
                write!(f, "jalr {args}")
            }
            Instruction::Jal { rd, imm } => {
                write!(f, "jal {}({})", *imm as i32, REG_NAMES[*rd as usize])
            }
            Instruction::ECall => {
                write!(f, "ecall")
            }
            Instruction::EBreak => {
                write!(f, "ebreak")
            }
            Instruction::SRet => {
                write!(f, "sret")
            }
            Instruction::MRet => {
                write!(f, "mret")
            }
            Instruction::SFenceVma => {
                write!(f, "sfence.vma")
            }
            Instruction::CsrRw { rd, rs1, imm } => {
                print_csr!(f, "csrw", "csrrw", rd, rs1, imm)
            }
            Instruction::CsrRs { rd, rs1, imm } => {
                print_csr!(f, "csrs", "csrrs", rd, rs1, imm)
            }
            Instruction::CsrRc { rd, rs1, imm } => {
                print_csr!(f, "csrc", "csrrc", rd, rs1, imm)
            }
            Instruction::CsrRwi { rd, rs1, imm } => {
                print_csr!(f, "csrwi", "csrrwi", rd, rs1, imm)
            }
            Instruction::CsrRsi { rd, rs1, imm } => {
                print_csr!(f, "csrsi", "csrrsi", rd, rs1, imm)
            }
            Instruction::CsrRci { rd, rs1, imm } => {
                print_csr!(f, "csrci", "csrrci", rd, rs1, imm)
            }
            Instruction::Fence { pred, succ } => {
                write!(f, "fence {}, {}", *pred as i32, *succ as i32)
            }
            Instruction::FenceI { pred, succ } => {
                write!(f, "fence.i {}, {}", *pred as i32, *succ as i32)
            }
            Instruction::Flw { rd, rs1, imm } => {
                write!(
                    f,
                    "flw {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Fsw { rs1, rs2, imm } => {
                write!(
                    f,
                    "fsw {}, {}({})",
                    REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Fld { rd, rs1, imm } => {
                write!(
                    f,
                    "fld {}, {}({})",
                    REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Fsd { rs1, rs2, imm } => {
                write!(
                    f,
                    "fsd {}, {}({})",
                    REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FmaddS { rd, rs1, rs2, rs3 } => {
                write!(
                    f,
                    "fmadd.s {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    REG_NAMES[*rs3 as usize]
                )
            }
            Instruction::FmsubS { rd, rs1, rs2, rs3 } => {
                write!(
                    f,
                    "fmsub.s {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    REG_NAMES[*rs3 as usize]
                )
            }
            Instruction::FnmaddS { rd, rs1, rs2, rs3 } => {
                write!(
                    f,
                    "fnmadd.s {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    REG_NAMES[*rs3 as usize]
                )
            }
            Instruction::FnmsubS { rd, rs1, rs2, rs3 } => {
                write!(
                    f,
                    "fnmsub.s {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    REG_NAMES[*rs3 as usize]
                )
            }
            Instruction::FaddS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fadd.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsubS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsub.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FmulS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fmul.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FdivS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fdiv.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsqrtS { rd, rs1 } => {
                write!(
                    f,
                    "fsqrt.s {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FsgnjS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsgnj.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsgnjnS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsgnjn.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsgnjxS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsgnjx.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FminS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fmin.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FmaxS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fmax.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FcvtSW { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.s.w {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtSWU { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.s.wu {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtWS { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.w.s {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtWUS { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.wu.s {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FmvXW { rd, rs1 } => {
                write!(
                    f,
                    "fmv.x.w {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FmvWX { rd, rs1 } => {
                write!(
                    f,
                    "fmv.w.x {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FeqS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "feq.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FltS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "flt.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FleS { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fle.s {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FClassS { rd, rs1 } => {
                write!(
                    f,
                    "fclass.s {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FaddD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fadd.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsubD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsub.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FmulD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fmul.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FdivD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fdiv.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsqrtD { rd, rs1 } => {
                write!(
                    f,
                    "fsqrt.d {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FsgnjD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsgnj.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsgnjnD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsgnjn.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FsgnjxD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fsgnjx.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FminD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fmin.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FmaxD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fmax.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FeqD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "feq.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FltD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "flt.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FleD { rd, rs1, rs2 } => {
                write!(
                    f,
                    "fle.d {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::FClassD { rd, rs1 } => {
                write!(
                    f,
                    "fclass.d {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtWD { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.w.d {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtWUD { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.wu.d {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtDW { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.d.w {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtDWU { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.d.wu {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtDS { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.d.s {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::FcvtSD { rd, rs1 } => {
                write!(
                    f,
                    "fcvt.s.d {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize]
                )
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                write!(
                    f,
                    "mul {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                write!(
                    f,
                    "mulh {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Mulsu { rd, rs1, rs2 } => {
                write!(
                    f,
                    "mulsu {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Mulu { rd, rs1, rs2 } => {
                write!(
                    f,
                    "mulu {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Div { rd, rs1, rs2 } => {
                write!(
                    f,
                    "div {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                write!(
                    f,
                    "divu {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                write!(
                    f,
                    "rem {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                write!(
                    f,
                    "remu {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize]
                )
            }
            Instruction::LrW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "lr.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::ScW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "sc.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmoswapW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amoswap.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmoaddW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amoadd.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmoandW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amoand.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmoorW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amoor.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmoxorW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amoxor.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmomaxW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amomax.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::AmominW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                write!(
                    f,
                    "amomin.w {}, {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    *rl as i32,
                    *aq as i32
                )
            }
            Instruction::CNop => {
                write!(f, "c.nop")
            }
            Instruction::CAddi4Spn { rd, nzuimm } => {
                write!(
                    f,
                    "c.addi4spn {}, {}",
                    REG_NAMES[*rd as usize], *nzuimm as i32
                )
            }
            Instruction::CSlli { rd, rs1, shamt } => {
                write!(
                    f,
                    "c.slli {}, {}, {}",
                    REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *shamt as i32
                )
            }
            Instruction::FmaddD { rd, rs1, rs2, rs3 } => {
                write!(
                    f,
                    "fmadd.d {}, {}, {}, {}",
                    REG_NAMES[*rd as usize],
                    REG_NAMES[*rs1 as usize],
                    REG_NAMES[*rs2 as usize],
                    REG_NAMES[*rs3 as usize]
                )
            }
			Instruction::FmsubD { rd, rs1, rs2, rs3 } => {
				write!(
					f,
					"fmsub.d {}, {}, {}, {}",
					REG_NAMES[*rd as usize],
					REG_NAMES[*rs1 as usize],
					REG_NAMES[*rs2 as usize],
					REG_NAMES[*rs3 as usize]
				)
			}
			Instruction::FnmsubD { rd, rs1, rs2, rs3 } => {
				write!(
					f,
					"fnmsub.d {}, {}, {}, {}",
					REG_NAMES[*rd as usize],
					REG_NAMES[*rs1 as usize],
					REG_NAMES[*rs2 as usize],
					REG_NAMES[*rs3 as usize]
				)
			}
			Instruction::FnmaddD { rd, rs1, rs2, rs3 } => {
				write!(
					f,
					"fnmadd.d {}, {}, {}, {}",
					REG_NAMES[*rd as usize],
					REG_NAMES[*rs1 as usize],
					REG_NAMES[*rs2 as usize],
					REG_NAMES[*rs3 as usize]
				)
			}
        }
    }
}
