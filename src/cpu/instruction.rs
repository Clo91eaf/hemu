#[derive(Copy, Clone)]
pub enum Instruction {
  Register(RegisterType),
  Immediate(ImmediateType),
  Store(StoreType),
  Branch(BranchType),
  Jump(JumpType),
  Upper(UpperType),
}

#[derive(Copy, Clone)]
pub enum RegisterType {
  ADD,
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

#[derive(Copy, Clone)]
pub enum ImmediateType {
  ADDI,
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
  ECALL,
  EBREAK,
}

#[derive(Copy, Clone)]
pub enum StoreType {
  SB,
  SH,
  SW,
  SD,
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
pub enum JumpType {
  JAL,
  JALR,
}

#[derive(Copy, Clone)]
pub enum UpperType {
  LUI,
  AUIPC,
}