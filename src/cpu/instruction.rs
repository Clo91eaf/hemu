enum Instruction {
  Branch(BranchType),
  Immediate(ImmediateType),
  Jump(JumpType),
  Upper(UpperType),
  Register(RegisterType),
  Store(StoreType),
  NoType,
}

enum BranchType {
  BEQ,
  BNE,
  BLT,
  BGE,
  BLTU,
  BGEU,
}

enum ImmediateType {
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

enum JumpType {
  JAL,
  JALR,
}

enum UpperType {
  LUI,
  AUIPC,
}

enum RegisterType {
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

enum StoreType {
  SB,
  SH,
  SW,
}
