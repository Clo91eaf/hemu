#[derive(Copy, Clone, Debug)]
pub enum Instruction {
  LoadStore(LoadStoreType),
  Computational(ComputationalType),
  JumpBranch(JumpBranchType),
  Miscellaneous(MiscellaneousType),
  Privileged(PrivilegedType),
}

#[derive(Copy, Clone, Debug)]
pub enum LoadStoreType {
  // Load
  LB,
  LBU,
  LH,
  LHU,
  LW,
  LWL,
  LWR,
  // Store
  SB,
  SH,
  SW,
  SWL,
  SWR,
  // Unaligned CPU Load and store
  LL,
  SC,
}

#[derive(Copy, Clone, Debug)]
pub enum ComputationalType {
  // immediate
  ADDI,
  ADDIU,
  ANDI,
  LUI,
  ORI,
  SLTI,
  SLTIU,
  XORI,
  // three register
  ADD,
  ADDU,
  AND,
  NOR,
  OR,
  SLT,
  SLTU,
  SUB,
  SUBU,
  XOR,
  // two operand
  CLO,
  CLZ,
  // shift instructions
  SLL,
  SLLV,
  SRA,
  SRAV,
  SRL,
  SRLV,
  // MULT/DIV
  DIV,
  DIVU,
  MADD,
  MADDU,
  MFHI,
  MFLO,
  MSUB,
  MSUBU,
  MTHI,
  MTLO,
  MUL,
  MULT,
  MULTU,
}

#[derive(Copy, Clone, Debug)]
pub enum JumpBranchType {
  // unconditional
  J,
  JAL,
  JALR,
  JR,
  // pc-relative conditianal branch comparing with register
  BEQ,
  BNE,
  // pc-relative conditianal branch comparing with zero
  BGEZ,
  BGEZAL,
  BGTZ,
  BLEZ,
  BLTZ,
  BLTZAL,
}

#[derive(Copy, Clone, Debug)]
pub enum MiscellaneousType {
  // exception
  BRAEAK,
  SYSCALL,
  // trap-on-condition instructions comparing two registers
  TEQ,
  TGE,
  TGEU,
  TLT,
  TLTU,
  TNE,
  // trap-on-condition instructions comparing a immediate value
  TEQI,
  TGEI,
  TGEIU,
  TLTI,
  TLTIU,
  TNEI,
  // CPU conditional move
  MOVF,
  MOVN,
  MOVT,
  MOVZ,
  // Prefetch
  PREF,
  // NOP
  NOP,
  SSNOP,
}

#[derive(Copy, Clone, Debug)]
pub enum PrivilegedType {
  CACHE,
  ERET,
  MFC0,
  MTC0,
  // TLB
  TLBP,
  TLBR,
  TLBWI,
  TLBWR,
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
