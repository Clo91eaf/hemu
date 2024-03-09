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
  // RV32M (RV64M)
  MUL,
  MULH,
  MULHSU,
  MULHU,
  DIV,
  DIVU,
  REM,
  REMU,
  // RV64I
  ADDW,
  SUBW,
  SLLW,
  SRLW,
  SRAW,

  MRET,
  // RV64M
  MULW,
  DIVW,
  DIVUW,
  REMW,
  REMUW,
}

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
}

#[derive(Copy, Clone, Debug)]
pub enum StoreType {
  // RV32I
  SB,
  SH,
  SW,
  // RV64I
  SD,
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

pub struct InstPattern {
  pub pattern: &'static str,
  pub itype: Instruction,
}

impl InstPattern {
  pub fn new(pattern: &'static str, itype: Instruction) -> InstPattern {
    InstPattern { pattern, itype }
  }
}
