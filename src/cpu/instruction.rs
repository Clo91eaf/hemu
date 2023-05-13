#[derive(Copy, Clone)]
pub enum Instruction {
  Branch(BranchType),
  Immediate(ImmediateType),
  Jump(JumpType),
  Upper(UpperType),
  Register(RegisterType),
  Store(StoreType),
  No(NoType),
}

#[derive(Copy, Clone)]
pub enum BranchType {
  BEQ,
  BNE,
  BLT,
  BGE,
  BLTU,
  BGEU,
}

#[derive(Copy, Clone)]
pub enum ImmediateType {
  ADDI,
  SLTI,
  SLTIU,
  XORI,
  ORI,
  ANDI,
  SLLI,
  SRLI,
  SRAI,
}

#[derive(Copy, Clone)]
pub enum JumpType {
  JAL,
  JALR,
}

#[derive(Copy, Clone)]
pub enum UpperType {
  LUI,
  AUIPC,
}

#[derive(Copy, Clone)]
pub enum RegisterType {
  ADD,
  SUB,
  SLL,
  SLT,
  SLTU,
  XOR,
  SRL,
  SRA,
  OR,
  AND,
}

#[derive(Copy, Clone)]
pub enum StoreType {
  SB,
  SH,
  SW,
}

#[derive(Copy, Clone)]
pub enum NoType {
  EBREAK,
}