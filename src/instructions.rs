use instruction_creator::instructions;

pub type InstructionSize = u32;
pub type SignedInstructionSize = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionFormat {
    RType,
    IType,
    SType,
    UType,
    BType,
    JType,
}

instructions! {
    // register
    add {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT7: u32 = 0;
    }
    sub {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT7: u32 = 32;
    }
    xor {
        pub const FUNCT3: u32 = 4;
        pub const FUNCT7: u32 = 0;
    }
    or {
        pub const FUNCT3: u32 = 6;
        pub const FUNCT7: u32 = 0;
    }
    and {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT7: u32 = 0;
    }
    sll {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT7: u32 = 0;
    }
    srl {
        pub const FUNCT3: u32 = 5;
        pub const FUNCT7: u32 = 0;
    }
    sra {
        pub const FUNCT3: u32 = 5;
        pub const FUNCT7: u32 = 32;
    }
    slt {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT7: u32 = 0;
    }
    sltu {
        pub const FUNCT3: u32 = 3;
        pub const FUNCT7: u32 = 0;
    }
    // immediate
    addi {
        pub const FUNCT3: u32 = 0;
    }
    xori {
        pub const FUNCT3: u32 = 4;
    }
    ori {
        pub const FUNCT3: u32 = 6;
    }
    andi {
        pub const FUNCT3: u32 = 7;
    }
    slli {
        pub const FUNCT3: u32 = 1;
        pub const IMM: u32 = 0;
    }
    srli {
        pub const FUNCT3: u32 = 5;
        pub const IMM: u32 = 0;
    }
    srai {
        pub const FUNCT3: u32 = 5;
        pub const IMM: u32 = 32;
    }
    slti {
        pub const FUNCT3: u32 = 2;
    }
    sltiu {
        pub const FUNCT3: u32 = 3;
    }
    jalr {
        pub const FUNCT3: u32 = 0;
    }
    // control
    ecall {
        pub const FUNCT3: u32 = 0;
        pub const IMM: u32 = 0;
    }
    sfencevma {
        pub const FUNCT3: u32 = 0;
        pub const IMM: u32 = 0x120;
    }
    ebreak {
        pub const FUNCT3: u32 = 0;
        pub const IMM: u32 = 1;
    }
    mret {
        pub const FUNCT3: u32 = 0;
        pub const IMM: u32 = 0x302;
    }
    sret {
        pub const FUNCT3: u32 = 0;
        pub const IMM: u32 = 0x102;
    }
    // M type
    mul {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT7: u32 = 1;
    }
    mulh {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT7: u32 = 1;
    }
    mulsu {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT7: u32 = 1;
    }
    mulu {
        pub const FUNCT3: u32 = 3;
        pub const FUNCT7: u32 = 1;
    }
    div {
        pub const FUNCT3: u32 = 4;
        pub const FUNCT7: u32 = 1;
    }
    divu {
        pub const FUNCT3: u32 = 5;
        pub const FUNCT7: u32 = 1;
    }
    rem {
        pub const FUNCT3: u32 = 6;
        pub const FUNCT7: u32 = 1;
    }
    remu {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT7: u32 = 1;
    }
    // load
    lb {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT7: u32 = 0;
    }
    lh {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT7: u32 = 0;
    }
    lw {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT7: u32 = 0;
    }
    lbu {
        pub const FUNCT3: u32 = 4;
        pub const FUNCT7: u32 = 0;
    }
    lhu {
        pub const FUNCT3: u32 = 5;
        pub const FUNCT7: u32 = 0;
    }
    // store
    sb {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT7: u32 = 0;
    }
    sh {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT7: u32 = 0;
    }
    sw {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT7: u32 = 0;
    }
    // branch
    beq {
        pub const FUNCT3: u32 = 0;
    }
    bne {
        pub const FUNCT3: u32 = 1;
    }
    blt {
        pub const FUNCT3: u32 = 4;
    }
    bge {
        pub const FUNCT3: u32 = 5;
    }
    bltu {
        pub const FUNCT3: u32 = 6;
    }
    bgeu {
        pub const FUNCT3: u32 = 7;
    }
    // jump
    jal { /* Nothing Here */ }
    // csr
    csrrw {
        pub const FUNCT3: u32 = 1;
    }
    csrrs {
        pub const FUNCT3: u32 = 2;
    }
    csrrc {
        pub const FUNCT3: u32 = 3;
    }
    csrrwi {
        pub const FUNCT3: u32 = 5;
    }
    csrrsi {
        pub const FUNCT3: u32 = 6;
    }
    csrrci {
        pub const FUNCT3: u32 = 7;
    }
    // fence
    fence {
        pub const FUNCT3: u32 = 0;
    }
    fence_i {
        pub const FUNCT3: u32 = 1;
    }
    // atomic
    lr_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 2;
    }
    sc_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 3;
    }
    amoswap_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 1;
    }
    amoadd_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 0;
    }
    amoand_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 12;
    }
    amoor_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 10;
    }
    amoxor_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 4;
    }
    amomax_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 20;
    }
    amomin_w {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 16;
    }

    // ????
    amominu_w {}
    amomaxu_w {}

    // F extention instructions
    fadd_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 0;
    }
    fsub_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 1;
    }
    fmul_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 2;
    }
    fdiv_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 3;
    }
    fsqrt_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 11;
    }
    fsgnj_s {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT5: u32 = 4;
    }
    fsgnjn_s {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT5: u32 = 4;
    }
    fsgnjx_s {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 4;
    }
    fmin_s {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT5: u32 = 5;
    }
    fmax_s {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT5: u32 = 5;
    }
    fcvt_w_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 24;
        pub const RS2: u32 = 0;
    }
    fcvt_wu_s {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 24;
        pub const RS2: u32 = 1;
    }
    fcvt_s_w {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 26;
    }
    fcvt_s_wu {
        pub const FUNCT3: u32 = 7;
        pub const FUNCT5: u32 = 27;
    }
    fmv_x_w {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT5: u32 = 28;
    }
    fmv_w_x {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT5: u32 = 30;
    }
    fle_s {
        pub const FUNCT3: u32 = 0;
        pub const FUNCT5: u32 = 20;
    }
    flt_s {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT5: u32 = 20;
    }
    feq_s {
        pub const FUNCT3: u32 = 2;
        pub const FUNCT5: u32 = 20;
    }
    fclass_s {
        pub const FUNCT3: u32 = 1;
        pub const FUNCT5: u32 = 28;
    }

    // utype
    lui { /* Nothing here */ }
    auipc { /* Nothing here */ }
}

pub mod compressed {
    use super::InstructionSize;

    pub type CompressedSize = u16;

    pub fn is_compressed(inst: InstructionSize) -> bool {
        const COMPRESSED_MASK: CompressedSize = 0b11;
        match (inst & 0xFFFF) as u16 & COMPRESSED_MASK {
            0 | 1 | 2 => true,
            _ => false,
        }
    }

    pub mod crtype {
        use super::CompressedSize;
        use bitfield::bitfield;

        bitfield! {
            pub struct CRType(CompressedSize);
            impl Debug;
            pub opcode, _: 1, 0;
            rs2, _: 6, 2; // must be 0
            rs1, _: 11, 7; // rs1 != 0
            pub funct4, _: 15, 12;
        }

        impl CRType {
            pub fn new(inst: CompressedSize) -> Self {
                Self(inst)
            }
        }

        #[test]
        fn crtype() {
            let inst = CRType(0x8602 /* c.jr x12 */);
            assert_eq!(inst.opcode(), 2);
            assert_eq!(inst.funct4(), 8);
            assert_eq!(inst.rs1(), 12);
            assert_eq!(inst.rs2(), 0);
        }
    }

    pub mod csstype {
        // TODO: Implement compressed S-Type
    }

    pub mod cwitype {
        // TODO: Implement compressed W-Type
    }

    pub mod citype {
        // TODO: Implement compressed I-Type
    }

    pub mod cjtype {
        // TODO: Implement compressed J-Type
    }

    pub mod cbtype {
        // TODO: Implement compressed B-Type
    }

    pub mod cltype {
        // TODO: Implement compressed L-Type
    }

    pub mod cstype {
        // TODO: Implement cs-type
    }
}

pub const LOAD_MATCH: InstructionSize = 3;
pub const FENCE_MATCH: InstructionSize = 15;
pub const ARITMETIC_IMMEDIATE_MATCH: InstructionSize = 19;
pub const AUIPC_MATCH: InstructionSize = 23;
pub const LUI_MATCH: InstructionSize = 55;
pub const STORE_MATCH: InstructionSize = 35;
pub const ARITMETIC_REGISTER_MATCH: InstructionSize = 51;

// TODO: maybe this is correct, check it
pub const FLOATING_POINT_MATCH: InstructionSize = 83;

pub const BRANCH_MATCH: InstructionSize = 99;
pub const CSR_MATCH: InstructionSize = 115;
pub const JALR_MATCH: InstructionSize = 103;
pub const JAL_MATCH: InstructionSize = 111;
pub const ATOMIC_MATCH: InstructionSize = 47;

pub mod rtype {
    use super::InstructionSize;
    use bitfield::bitfield;

    bitfield! {
        pub struct RType(InstructionSize);
        impl Debug;
        InstructionSize;
        pub opcode, _: 6, 0;
        pub rd, _:     11, 7;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub rs2, _:    24, 20;
        pub funct7, _: 31, 25;
    }

    impl RType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }
    }

    #[test]
    fn atomic_check() {
        let inst = RType(0xCF4A7AF /* amoswap.w x15, x15, (x9) */);
        assert_eq!(inst.rd(), 15);
        assert_eq!(inst.rs1(), 9);
        assert_eq!(inst.rs2(), 15);
    }
}

pub mod itype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct IType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub rd, _:     11, 7;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub uimm, _:   31, 20;
        SignedInstructionSize;
        imm_signed, _:   31, 20;
    }

    impl IType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        pub fn imm(&self) -> InstructionSize {
            self.imm_signed() as InstructionSize
        }
    }

    #[test]
    fn csr_check() {
        let inst = IType(0xf14025f3 /* csrrs x11, mhartid, x0 */);
        assert_eq!(inst.rd(), 11);
        assert_eq!(inst.rs1(), 0);
        assert_eq!(inst.uimm(), 3860);
        let inst = IType(0x30101773 /* csrrw x14, misa, x0 */);
        assert_eq!(inst.rd(), 14);
        assert_eq!(inst.rs1(), 0);
        assert_eq!(inst.uimm(), 769);
        let inst = IType(0xc0006073 /* csrrsi _, _, _ */);
        assert_eq!(inst.rd(), 0);
        assert_eq!(inst.rs1(), 0);
        assert_eq!(inst.uimm(), 0b1100_0000_0000);
    }

    #[test]
    fn imm_check() {
        let inst = IType(0x06468613 /* addi x12 x13 100 */);
        assert_eq!(inst.rd(), 12);
        assert_eq!(inst.rs1(), 13);
        assert_eq!(inst.imm(), 100);
    }

    #[test]
    fn instructions() {
        let inst = IType(0x00411573 /* csrrw x10 x2 4 */);
        assert_eq!(inst.rd(), 10);
        assert_eq!(inst.rs1(), 2);
        assert_eq!(inst.imm(), 4);
        let inst = IType(0x00c12603 /* lw x12, 12(sp) */);
        assert_eq!(inst.rd(), 12);
        assert_eq!(inst.rs1(), 2);
        assert_eq!(inst.imm(), 12);
        let inst = IType(0x00c080e7 /* jalr x1, 12(ra) */);
        assert_eq!(inst.rd(), 1);
        assert_eq!(inst.rs1(), 1);
        assert_eq!(inst.imm(), 12);
        let inst = IType(0x379793 /* slli a5, a5, 3 */);
        assert_eq!(inst.rd(), 15);
        assert_eq!(inst.rs1(), 15);
        assert_eq!(inst.imm(), 3);
        use crate::{decoded_inst::InstructionDecoded, decoder::try_decode};
        let inst = try_decode(0x379793 /* slli a5, a5, 3 */).unwrap();
        assert!(matches!(
            inst,
            InstructionDecoded::Slli {
                rd: 15,
                rs1: 15,
                imm: 3
            }
        ));
        let inst = IType(0x00197793 /* andi a5, s2, 1 */);
        assert_eq!(inst.rd(), 15);
        assert_eq!(inst.rs1(), 18);
        assert_eq!(inst.imm(), 1);
    }
}

pub mod stype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct SType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub imm1, _:   11, 7;
        InstructionSize;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub rs2, _:    24, 20;
        SignedInstructionSize;
        pub imm2, _:   31, 25;
    }

    impl SType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        pub fn imm(&self) -> InstructionSize {
            self.imm1() | (self.imm2() << 5) as InstructionSize
        }
    }

    #[test]
    fn imm_check() {
        let inst = SType(0x00112f23 /* sw ra, 30(sp) */);
        assert_eq!(inst.rs1(), 2);
        assert_eq!(inst.rs2(), 1);
        assert_eq!(inst.imm(), 30);
    }
}

pub mod utype {
    use super::InstructionSize;
    use bitfield::bitfield;

    bitfield! {
        pub struct UType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub rd, _:     11, 7;
        // SignedInstructionSize;
        pub imm, _:   31, 12;
    }

    impl UType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        // pub fn imm(&self) -> InstructionSize {
        //     self.imm1() as InstructionSize
        // }
    }

    #[test]
    pub fn imm_check() {
        let inst = UType(0x00004537 /* lui x10, 4 */);
        assert_eq!(inst.rd(), 10);
        assert_eq!(inst.imm(), 4);
    }
}

// aims to mimic `mm[12|10:5] rs2 rs1 funct3 imm[4:1|11] opcode B-type` in the RISC-V spec
pub mod btype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct BType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub imm1, _:   7, 7;
        pub imm2, _:   11, 8;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub rs2, _:    24, 20;
        pub imm3, _:   30, 25;
        SignedInstructionSize;
        pub imm4, _:   31, 31;
    }

    impl BType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        pub fn imm(&self) -> InstructionSize {
            let (imm1, imm2, imm3, imm4) = (
                self.imm1() << 11,
                self.imm2() << 1,
                self.imm3() << 5,
                self.imm4() << 12,
            );
            imm1 | imm2 | imm3 | imm4 as InstructionSize
        }
    }

    #[test]
    fn imm_check() {
        let inst = BType(0x50A60463 /* beq x12 x10 1288 */);
        assert_eq!(inst.rs1(), 12);
        assert_eq!(inst.rs2(), 10);
        assert_eq!(inst.imm(), 1288);
        let inst = BType(0x00409663 /* bne x1 x4 12 */);
        assert_eq!(inst.rs1(), 1);
        assert_eq!(inst.rs2(), 4);
        assert_eq!(inst.imm(), 12);
        let inst = BType(0xfe078ce3 /* beq x15, x0, -8 */);
        assert_eq!(inst.rs1(), 15);
        assert_eq!(inst.rs2(), 0);
        assert_eq!(inst.imm() as SignedInstructionSize, -8);
        let inst = BType(0xfe20dae3 /* bge x1 x2 -12 */);
        assert_eq!(inst.rs1(), 1);
        assert_eq!(inst.rs2(), 2);
        assert_eq!(inst.imm() as SignedInstructionSize, -12);
    }
}

pub mod jtype {
    use crate::bit_ops;

    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct JType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub rd, _: 11, 7;
    }

    impl JType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        fn imm1(&self) -> InstructionSize {
            let imm = (bit_ops::get_bit(self.0, 31) << 31) as SignedInstructionSize;
            (imm >> 11) as InstructionSize
        }

        fn imm2(&self) -> InstructionSize {
            bit_ops::get_bits(self.0, 8, 12) << 12
        }

        fn imm3(&self) -> InstructionSize {
            let imm = self.0 >> 9 /* now get bit 11 */;
            bit_ops::get_bit(imm, 11) << 11
        }

        fn imm4(&self) -> InstructionSize {
            let imm = self.0 >> 20 /* now get bits 10:1 */;
            bit_ops::get_bits(imm, 10, 1) << 1
        }

        pub fn imm(&self) -> InstructionSize {
            let (imm1, imm2, imm3, imm4) = (
                self.imm1(), // imm[20]
                self.imm2(), // imm[19:12]
                self.imm3(), // imm[11]
                self.imm4(), // imm[10:1]
            );
            imm1 | imm2 | imm3 | imm4
        }
    }

    #[test]
    fn imm_check() {
        let inst = JType(0x0100006f /* jal x0 16 */);
        assert_eq!(inst.rd(), 0);
        assert_eq!(inst.imm(), 16);
        let inst = JType(
            0x84000EF, /* JAL ra 132 (0b00001000010000000000000011101111) */
        );
        assert_eq!(inst.rd(), 1);
        assert_eq!(inst.imm(), 132);
        let inst = JType(0xfb9ff0ef /* jal ra, -72 */);
        assert_eq!(inst.rd(), 1);
        assert_eq!(inst.imm() as SignedInstructionSize, -72);
    }
}
