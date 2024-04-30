mod rv64i;
mod rv64m;
mod rv64a;
mod rv64f;
mod rv64d;
use rv64i::rv64i;
use rv64m::rv64m;
use rv64a::rv64a;
use rv64f::rv64f;
use rv64d::rv64d;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
  Register(RegisterType),
  Immediate(ImmediateType),
  Store(StoreType),
  Branch(BranchType),
  Jump(JumpType),
  Upper(UpperType),
  ERROR,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum RegisterType {
  // RV32I
  ADD,
  SUB,
  XOR,
  OR,
  AND,
  SLL,
  SRL,
  SRA,
  SLT,
  SLTU,
  // RV32M
  MUL,
  MULH,
  MULHSU,
  MULHU,
  DIV,
  DIVU,
  REM,
  REMU,

  // RV64M
  MULW,
  DIVW,
  DIVUW,
  REMW,
  REMUW,

  // RV64I
  ADDW,
  SUBW,
  SLLW,
  SRLW,
  SRAW,

  SRET,
  MRET,

  SFENCE_VMA,
  // Privileged Spec
  WFI,

  // RV32A
  LR_W,
  SC_W,
  AMOSWAP_W,
  AMOADD_W,
  AMOXOR_W,
  AMOAND_W,
  AMOOR_W,
  AMOMIN_W,
  AMOMAX_W,
  AMONINU_W,
  AMOMAXU_W,

  // RV64A
  LR_D,
  SC_D,
  AMOSWAP_D,
  AMOADD_D,
  AMOXOR_D,
  AMOAND_D,
  AMOOR_D,
  AMOMIN_D,
  AMOMAX_D,
  AMOMINU_D,
  AMOMAXU_D,

  // RV32F
  FADD_S,
  FSUB_S,
  FMUL_S,
  FDIV_S,
  FSQRT_S,
  FSGNJ_S,
  FSGNJN_S,
  FSGNJX_S,
  FMIN_S,
  FMAX_S,
  FCVT_W_S,
  FMV_X_W,
  FEQ_S,
  FLT_S,
  FLE_S,
  FCLASS_S,
  FCVT_S_W,

  // RV64F
  FCVT_L_S,
  FCVT_LU_S,
  FCVT_S_L,
  FCVT_S_LU,

  // RV32D
  FADD_D,
  FSUB_D,
  FMUL_D,
  FDIV_D,
  FSQRT_D,
  FSGNJ_D,
  FSGNJN_D,
  FSGNJX_D,
  FMIN_D,
  FMAX_D,
  FCVT_S_D,
  FCVT_D_S,
  FEQ_D,
  FLT_D,
  FLE_D,
  FCLASS_D,
  FCVT_W_D,
  FMV_D_W,

  // RV64D
  FCVT_L_D,
  FCVT_LU_D,
  FMV_X_D,
  FCVT_D_L,
  FCVT_D_LU,
  FMV_D_X,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum ImmediateType {
  // RV32I
  ADDI,
  XORI,
  ORI,
  ANDI,
  SLLI, // both rv64
  SRLI, // both rv64
  SRAI, // both rv64
  SLTI,
  SLTIU,

  LB,
  LH,
  LW,
  LBU,
  LHU,

  JALR,

  ECALL,
  EBREAK,
  FENCE,
  FENCE_I,

  CSRRW,
  CSRRS,
  CSRRC,
  CSRRWI,
  CSRRSI,
  CSRRCI,
  // RV64I
  LWU,
  LD,
  ADDIW,
  SLLIW,
  SRLIW,
  SRAIW,

  // RV32F
  FLW,
  // RV32D
  FLD,
}

#[derive(Copy, Clone, Debug)]
pub enum StoreType {
  // RV32I
  SB,
  SH,
  SW,
  // RV64I
  SD,
  // RV64F
  FSW,
  // RV64D
  FSD,
}

#[derive(Copy, Clone, Debug)]
pub enum BranchType {
  // RV32I
  BEQ,
  BNE,
  BLT,
  BGE,
  BLTU,
  BGEU,
}

#[derive(Copy, Clone, Debug)]
pub enum JumpType {
  // RV32I
  JAL,
}

#[derive(Copy, Clone, Debug)]
pub enum UpperType {
  // RV32I
  LUI,
  AUIPC,
}

fn bitmask(bits: u32) -> u32 {
  (1u32 << bits) - 1
}

// [lo, hi)
pub fn bits(x: u32, lo: u32, hi: u32) -> usize {
  assert!(hi >= lo);
  ((x >> lo) & bitmask(hi - lo)) as usize
}

pub fn sext(x: usize, len: u32) -> u64 {
  assert!(len <= 64);
  let extend_bits = 64 - len;
  (((x as i64) << extend_bits) >> extend_bits) as u64
}

/// Instruction
pub struct Inst {
  /// instruction bits
  pub bits: u32,
  /// instruction type
  pub typ: Instruction,
  /// instruction pattern table
  ipt: Vec<InstPattern>,
  /// instruction type name
  pub type_name: &'static str,
  pub rd: usize,
  pub rs1: usize,
  pub rs2: usize,
  pub imm: i64,
}

impl Inst {
  pub fn new() -> Inst {
    // new ipt
    let mut ipt :Vec<InstPattern>= vec![];
    ipt.extend(rv64i());
    ipt.extend(rv64m());
    ipt.extend(rv64a());
    ipt.extend(rv64f());
    ipt.extend(rv64d());

    // self
    Inst {
      bits: 0,
      typ: Instruction::ERROR,
      type_name: "",
      ipt,
      rd: 0,
      rs1: 0,
      rs2: 0,
      imm: 0,
    }
  }

  pub fn set_bits<T: Into<u32>>(&mut self, bits: T) -> anyhow::Result<()> {
    self.bits = bits.into();
    self.decode()?;
    (self.rd, self.rs1, self.rs2, self.imm) = self.decode_operand().unwrap();
    Ok(())
  }

  #[rustfmt::skip]
  pub fn decode_operand(&self) -> Option<(usize, usize, usize, i64)> {
    let (rd, rs1, rs2) = (bits(self.bits, 7, 12), bits(self.bits, 15, 20), bits(self.bits, 20, 25));
    match self.typ {
      Instruction::Register(_) => Some((rd, rs1, rs2, 0)),
      Instruction::Immediate(_) => Some((rd, rs1, 0, 
        sext(bits(self.bits, 20, 32), 12) as i64)),
      Instruction::Store(_) => Some((0, rs1, rs2, 
        sext(bits(self.bits, 25, 32) << 5 | bits(self.bits, 7, 12), 12) as i64)),
      Instruction::Branch(_) => Some((0, rs1, rs2, 
        sext(bits(self.bits, 31, 32) << 12 | bits(self.bits, 7, 8) << 11 | bits(self.bits, 25, 31) << 5 | bits(self.bits, 8, 12) << 1, 13) as i64)),
      Instruction::Upper(_) => Some((rd, 0, 0, 
        sext(bits(self.bits, 12, 32) << 12, 32) as i64)),
      Instruction::Jump(_) => Some((rd, 0, 0, 
        sext(bits(self.bits, 31, 32) << 20 | bits(self.bits, 21, 31) << 1 | bits(self.bits, 20, 21) << 11 | bits(self.bits, 12, 20) << 12, 21,) as i64)),
      Instruction::ERROR => {None}
    }
  }

  fn match_inst(&self, pattern: &str) -> bool {
    let mut mask = 0;
    let mut expected = 0;

    for (i, c) in pattern.replace(" ", "").chars().enumerate() {
      if c == '0' || c == '1' {
        mask |= 1 << (31 - i);
        expected |= (c.to_digit(2).unwrap() as u32) << (31 - i);
      }
    }

    (self.bits & mask) == expected
  }

  fn decode(&mut self) -> anyhow::Result<()> {
    // todo: this can be optimized by map
    let instruction_pattern = self
      .ipt
      .iter()
      .find(|p| self.match_inst(p.pattern))
      .ok_or_else(|| {anyhow::anyhow!("Unknown instruction")})?;

    self.typ = instruction_pattern.itype;
    self.type_name = instruction_pattern.name;

    Ok(())
  }
}

impl fmt::Display for Inst {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (rd, rs1, rs2, imm) = (self.rd, self.rs1, self.rs2, self.imm);
    let rd = rd as usize;
    let gpr = vec!["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
    let output = match self.typ {
      Instruction::Register(_)  => format!("{} {}, {}, {}", self.type_name, gpr[rd], gpr[rs1], gpr[rs2]),
      Instruction::Immediate(_) => format!("{} {}, {}, {}", self.type_name, gpr[rd], gpr[rs1], imm),
      Instruction::Store(_)     => format!("{} {}, {}({})", self.type_name, gpr[rs2], imm, gpr[rs1]),
      Instruction::Branch(_)    => format!("{} {}, {}, pc + ({:x})", self.type_name, gpr[rs1], gpr[rs2], imm),
      Instruction::Jump(_)      => format!("{} {}, pc + ({:x})", self.type_name, gpr[rd], imm),
      Instruction::Upper(_)     => format!("{} {}, {:x}", self.type_name, gpr[rd], imm),
      _ => format!("Unknown instruction")
    };
    write!(f, "{}", output)
  }
}

pub struct InstPattern {
  pub name: &'static str,
  pub pattern: &'static str,
  pub itype: Instruction,
}

impl InstPattern {
  pub fn new(name: &'static str, pattern: &'static str, itype: Instruction) -> InstPattern {
    InstPattern { name, pattern, itype }
  }
}
