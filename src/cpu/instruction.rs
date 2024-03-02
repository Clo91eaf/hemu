#[derive(Copy, Clone, Debug)]
pub enum Instruction {
  Register(RegisterType),
  Immediate(ImmediateType),
  Store(StoreType),
  Branch(BranchType),
  Jump(JumpType),
  Upper(UpperType),
}

#[derive(Copy, Clone, Debug)]
pub enum RegisterType {
  ADD,
  ADDW,
  SUB,
  XOR,
  OR,
  AND,
  SLL,
  SRL,
  // SRA, // todo
  SLT,
  SLTU,
  MUL,
  MULW,
  DIV,
  DIVU,
  REM,
  REMU,
}

#[derive(Copy, Clone, Debug)]
pub enum ImmediateType {
  ADDI,
  ADDIW,
  XORI,
  ORI,
  ANDI,
  SLLI,
  SRLI,
  SRAI,
  SLTI,
  SLTIU,
  LB,
  LH,
  LW,
  LD,
  LBU,
  LHU,
  LWU,
  LDU,
  JALR,
  ECALL,
  EBREAK,
}

#[derive(Copy, Clone, Debug)]
pub enum StoreType {
  SB,
  SH,
  SW,
  SD,
}

#[derive(Copy, Clone, Debug)]
pub enum BranchType {
  BEQ,
  BNE,
  BLT,
  BGE,
  BLTU,
  BGEU,
}

#[derive(Copy, Clone, Debug)]
pub enum JumpType {
  JAL,
}

#[derive(Copy, Clone, Debug)]
pub enum UpperType {
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
