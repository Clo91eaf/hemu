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
    Inst {
      bits: 0,
      typ: Instruction::ERROR,
      type_name: "",
      ipt: new_ipt(),
      rd: 0,
      rs1: 0,
      rs2: 0,
      imm: 0,
    }
  }

  pub fn set_bits<T: Into<u32>>(&mut self, bits: T) {
    self.bits = bits.into();
    self.decode().unwrap();
    (self.rd, self.rs1, self.rs2, self.imm) = self.decode_operand().unwrap();
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

  pub fn disassemble(&self, pc: u64) -> String {
    let (rd, rs1, rs2, imm) = (self.rd, self.rs1, self.rs2, self.imm);
    let rd = rd as usize;
    let gpr = vec!["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
    match self.typ {
      Instruction::Register(_)  => format!("{} {}, {}, {}", self.type_name, gpr[rd], gpr[rs1], gpr[rs2]),
      Instruction::Immediate(_) => format!("{} {}, {}, {}", self.type_name, gpr[rd], gpr[rs1], imm),
      Instruction::Store(_)     => format!("{} {}, {}({})", self.type_name, gpr[rs2], imm, gpr[rs1]),
      Instruction::Branch(_)    => format!("{} {}, {}, {:x}", self.type_name, gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
      Instruction::Jump(_)      => format!("{} {}, {:x}", self.type_name, gpr[rd], (imm as u64).wrapping_add(pc)),
      Instruction::Upper(_)     => format!("{} {}, {:x}", self.type_name, gpr[rd], imm),
      _ => format!("Unknown instruction")
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
    let instruction_pattern = self
      .ipt
      .iter()
      .find(|p| self.match_inst(p.pattern))
      .unwrap();

    self.typ = instruction_pattern.itype;
    self.type_name = instruction_pattern.name;

    Ok(())
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

#[rustfmt::skip]
pub fn new_ipt() -> Vec<InstPattern> {
    vec![
  // RegisterType
  // rv32I 
  InstPattern::new("add", "0000000 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::ADD)),
  InstPattern::new("sub", "0100000 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::SUB)),
  InstPattern::new("xor", "0000000 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::XOR)),
  InstPattern::new("or", "0000000 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::OR)),
  InstPattern::new("and", "0000000 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::AND)),
  InstPattern::new("sll", "0000000 ????? ????? 001 ????? 01100 11", Instruction::Register(RegisterType::SLL)),
  InstPattern::new("srl", "0000000 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::SRL)),
  InstPattern::new("sra", "0100000 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::SRA)),
  InstPattern::new("slt", "0000000 ????? ????? 010 ????? 01100 11", Instruction::Register(RegisterType::SLT)),
  InstPattern::new("sltu", "0000000 ????? ????? 011 ????? 01100 11", Instruction::Register(RegisterType::SLTU)),
  // rv32M
  InstPattern::new("mul", "0000001 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::MUL)),
  InstPattern::new("mulh", "0000001 ????? ????? 001 ????? 01100 11", Instruction::Register(RegisterType::MULH)),
  InstPattern::new("mulhsu", "0000001 ????? ????? 010 ????? 01100 11", Instruction::Register(RegisterType::MULHSU)),
  InstPattern::new("mulhu", "0000001 ????? ????? 011 ????? 01100 11", Instruction::Register(RegisterType::MULHU)),
  InstPattern::new("div", "0000001 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::DIV)),
  InstPattern::new("divu", "0000001 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::DIVU)),
  InstPattern::new("rem", "0000001 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::REM)),
  InstPattern::new("remu", "0000001 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::REMU)),
  // rv64I
  InstPattern::new("addw", "0000000 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::ADDW)),
  InstPattern::new("subw", "0100000 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::SUBW)),
  InstPattern::new("sllw", "0000000 ????? ????? 001 ????? 01110 11", Instruction::Register(RegisterType::SLLW)),
  InstPattern::new("srlw", "0000000 ????? ????? 101 ????? 01110 11", Instruction::Register(RegisterType::SRLW)),
  InstPattern::new("sraw", "0100000 ????? ????? 101 ????? 01110 11", Instruction::Register(RegisterType::SRAW)),

  InstPattern::new("mret", "0011000 00010 00000 000 00000 11100 11", Instruction::Register(RegisterType::MRET)),
  // rv64M
  InstPattern::new("mulw", "0000001 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::MULW)),
  InstPattern::new("divw", "0000001 ????? ????? 100 ????? 01110 11", Instruction::Register(RegisterType::DIVW)),
  InstPattern::new("divuw", "0000001 ????? ????? 100 ????? 01110 11", Instruction::Register(RegisterType::DIVUW)),
  InstPattern::new("remw", "0000001 ????? ????? 110 ????? 01110 11", Instruction::Register(RegisterType::REMW)),
  InstPattern::new("remuw", "0000001 ????? ????? 110 ????? 01110 11", Instruction::Register(RegisterType::REMUW)),

  // Immediate
  // rv32I
  InstPattern::new("addi", "??????? ????? ????? 000 ????? 00100 11", Instruction::Immediate(ImmediateType::ADDI)),
  InstPattern::new("xori", "??????? ????? ????? 100 ????? 00100 11", Instruction::Immediate(ImmediateType::XORI)),
  InstPattern::new("ori", "??????? ????? ????? 110 ????? 00100 11", Instruction::Immediate(ImmediateType::ORI)),
  InstPattern::new("andi", "??????? ????? ????? 111 ????? 00100 11", Instruction::Immediate(ImmediateType::ANDI)),
  InstPattern::new("slli", "000000? ????? ????? 001 ????? 00100 11", Instruction::Immediate(ImmediateType::SLLI)),
  InstPattern::new("srli", "000000? ????? ????? 101 ????? 00100 11", Instruction::Immediate(ImmediateType::SRLI)),
  InstPattern::new("srai", "010000? ????? ????? 101 ????? 00100 11", Instruction::Immediate(ImmediateType::SRAI)),
  InstPattern::new("slti", "??????? ????? ????? 010 ????? 00100 11", Instruction::Immediate(ImmediateType::SLTI)),
  InstPattern::new("sltiu", "??????? ????? ????? 011 ????? 00100 11", Instruction::Immediate(ImmediateType::SLTIU)),

  InstPattern::new("lb", "??????? ????? ????? 000 ????? 00000 11", Instruction::Immediate(ImmediateType::LB)),
  InstPattern::new("lh", "??????? ????? ????? 001 ????? 00000 11", Instruction::Immediate(ImmediateType::LH)),
  InstPattern::new("lw", "??????? ????? ????? 010 ????? 00000 11", Instruction::Immediate(ImmediateType::LW)),
  InstPattern::new("lbu", "??????? ????? ????? 100 ????? 00000 11", Instruction::Immediate(ImmediateType::LBU)),
  InstPattern::new("lhu", "??????? ????? ????? 101 ????? 00000 11", Instruction::Immediate(ImmediateType::LHU)),

  InstPattern::new("jalr", "??????? ????? ????? 000 ????? 11001 11", Instruction::Immediate(ImmediateType::JALR)),

  InstPattern::new("ebreak", "0000000 00001 00000 000 00000 11100 11", Instruction::Immediate(ImmediateType::EBREAK)),
  InstPattern::new("ecall", "0000000 00000 00000 000 00000 11100 11", Instruction::Immediate(ImmediateType::ECALL)),
  InstPattern::new("fence", "0000??? ????? 00000 000 00000 00011 11", Instruction::Immediate(ImmediateType::FENCE)),

  InstPattern::new("csrrw", "??????? ????? ????? 001 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRW)),
  InstPattern::new("csrrs", "??????? ????? ????? 010 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRS)),
  InstPattern::new("csrrc", "??????? ????? ????? 011 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRC)),
  InstPattern::new("csrrwi", "??????? ????? ????? 101 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRWI)),
  InstPattern::new("csrrsi", "??????? ????? ????? 110 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRSI)),
  InstPattern::new("csrrci", "??????? ????? ????? 111 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRCI)),
  // rv64I
  InstPattern::new("lwu", "??????? ????? ????? 110 ????? 00000 11", Instruction::Immediate(ImmediateType::LWU)),
  InstPattern::new("ld", "??????? ????? ????? 011 ????? 00000 11", Instruction::Immediate(ImmediateType::LD)),
  InstPattern::new("addiw", "??????? ????? ????? 000 ????? 00110 11", Instruction::Immediate(ImmediateType::ADDIW)),
  InstPattern::new("slliw", "000000? ????? ????? 001 ????? 00110 11", Instruction::Immediate(ImmediateType::SLLIW)),
  InstPattern::new("srliw", "000000? ????? ????? 101 ????? 00110 11", Instruction::Immediate(ImmediateType::SRLIW)),
  InstPattern::new("sraiw", "010000? ????? ????? 101 ????? 00110 11", Instruction::Immediate(ImmediateType::SRAIW)),

  // Store
  // rv32I
  InstPattern::new("sb", "??????? ????? ????? 000 ????? 01000 11", Instruction::Store(StoreType::SB)),
  InstPattern::new("sh", "??????? ????? ????? 001 ????? 01000 11", Instruction::Store(StoreType::SH)),
  InstPattern::new("sw", "??????? ????? ????? 010 ????? 01000 11", Instruction::Store(StoreType::SW)),
  // rv64I
  InstPattern::new("sd", "??????? ????? ????? 011 ????? 01000 11", Instruction::Store(StoreType::SD)),
  // Branch
  // rv32I
  InstPattern::new("beq", "??????? ????? ????? 000 ????? 11000 11", Instruction::Branch(BranchType::BEQ)),
  InstPattern::new("bne", "??????? ????? ????? 001 ????? 11000 11", Instruction::Branch(BranchType::BNE)),
  InstPattern::new("blt", "??????? ????? ????? 100 ????? 11000 11", Instruction::Branch(BranchType::BLT)),
  InstPattern::new("bge", "??????? ????? ????? 101 ????? 11000 11", Instruction::Branch(BranchType::BGE)),
  InstPattern::new("bltu", "??????? ????? ????? 110 ????? 11000 11", Instruction::Branch(BranchType::BLTU)),
  InstPattern::new("bgeu", "??????? ????? ????? 111 ????? 11000 11", Instruction::Branch(BranchType::BGEU)),
  // Jump
  // rv32I
  InstPattern::new("jal", "??????? ????? ????? ??? ????? 11011 11", Instruction::Jump(JumpType::JAL)),
  // Upper
  // rv32I
  InstPattern::new("lui", "??????? ????? ????? ??? ????? 01101 11", Instruction::Upper(UpperType::LUI)),
  InstPattern::new("auipc", "??????? ????? ????? ??? ????? 00101 11", Instruction::Upper(UpperType::AUIPC)),]
}
