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
      ipt: new_ipt(),
      rd: 0,
      rs1: 0,
      rs2: 0,
      imm: 0,
    }
  }

  pub fn set_bits(&mut self, bits: u32) {
    self.bits = bits;
    self.typ = self.decode();
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
    disassemble(self, pc)
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

  fn decode(&self) -> Instruction {
    self
      .ipt
      .iter()
      .find(|p| self.match_inst(p.pattern))
      .unwrap_or(&InstPattern::new("", Instruction::ERROR))
      .itype
  }
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

#[rustfmt::skip]
pub fn new_ipt() -> Vec<InstPattern> {
    vec![
  // RegisterType
  // rv32I 
  InstPattern::new("0000000 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::ADD)),
  InstPattern::new("0100000 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::SUB)),
  InstPattern::new("0000000 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::XOR)),
  InstPattern::new("0000000 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::OR)),
  InstPattern::new("0000000 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::AND)),
  InstPattern::new("0000000 ????? ????? 001 ????? 01100 11", Instruction::Register(RegisterType::SLL)),
  InstPattern::new("0000000 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::SRL)),
  InstPattern::new("0100000 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::SRA)),
  InstPattern::new("0000000 ????? ????? 010 ????? 01100 11", Instruction::Register(RegisterType::SLT)),
  InstPattern::new("0000000 ????? ????? 011 ????? 01100 11", Instruction::Register(RegisterType::SLTU)),
  // rv32M
  InstPattern::new("0000001 ????? ????? 000 ????? 01100 11", Instruction::Register(RegisterType::MUL)),
  InstPattern::new("0000001 ????? ????? 001 ????? 01100 11", Instruction::Register(RegisterType::MULH)),
  InstPattern::new("0000001 ????? ????? 010 ????? 01100 11", Instruction::Register(RegisterType::MULHSU)),
  InstPattern::new("0000001 ????? ????? 011 ????? 01100 11", Instruction::Register(RegisterType::MULHU)),
  InstPattern::new("0000001 ????? ????? 100 ????? 01100 11", Instruction::Register(RegisterType::DIV)),
  InstPattern::new("0000001 ????? ????? 101 ????? 01100 11", Instruction::Register(RegisterType::DIVU)),
  InstPattern::new("0000001 ????? ????? 110 ????? 01100 11", Instruction::Register(RegisterType::REM)),
  InstPattern::new("0000001 ????? ????? 111 ????? 01100 11", Instruction::Register(RegisterType::REMU)),
  // rv64I
  InstPattern::new("0000000 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::ADDW)),
  InstPattern::new("0100000 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::SUBW)),
  InstPattern::new("0000000 ????? ????? 001 ????? 01110 11", Instruction::Register(RegisterType::SLLW)),
  InstPattern::new("0000000 ????? ????? 101 ????? 01110 11", Instruction::Register(RegisterType::SRLW)),
  InstPattern::new("0100000 ????? ????? 101 ????? 01110 11", Instruction::Register(RegisterType::SRAW)),

  InstPattern::new("0011000 00010 00000 000 00000 11100 11", Instruction::Register(RegisterType::MRET)),
  // rv64M
  InstPattern::new("0000001 ????? ????? 000 ????? 01110 11", Instruction::Register(RegisterType::MULW)),
  InstPattern::new("0000001 ????? ????? 100 ????? 01110 11", Instruction::Register(RegisterType::DIVW)),
  InstPattern::new("0000001 ????? ????? 100 ????? 01110 11", Instruction::Register(RegisterType::DIVUW)),
  InstPattern::new("0000001 ????? ????? 110 ????? 01110 11", Instruction::Register(RegisterType::REMW)),
  InstPattern::new("0000001 ????? ????? 110 ????? 01110 11", Instruction::Register(RegisterType::REMUW)),

  // Immediate
  // rv32I
  InstPattern::new("??????? ????? ????? 000 ????? 00100 11", Instruction::Immediate(ImmediateType::ADDI)),
  InstPattern::new("??????? ????? ????? 100 ????? 00100 11", Instruction::Immediate(ImmediateType::XORI)),
  InstPattern::new("??????? ????? ????? 110 ????? 00100 11", Instruction::Immediate(ImmediateType::ORI)),
  InstPattern::new("??????? ????? ????? 111 ????? 00100 11", Instruction::Immediate(ImmediateType::ANDI)),
  InstPattern::new("000000? ????? ????? 001 ????? 00100 11", Instruction::Immediate(ImmediateType::SLLI)),
  InstPattern::new("000000? ????? ????? 101 ????? 00100 11", Instruction::Immediate(ImmediateType::SRLI)),
  InstPattern::new("010000? ????? ????? 101 ????? 00100 11", Instruction::Immediate(ImmediateType::SRAI)),
  InstPattern::new("??????? ????? ????? 010 ????? 00100 11", Instruction::Immediate(ImmediateType::SLTI)),
  InstPattern::new("??????? ????? ????? 011 ????? 00100 11", Instruction::Immediate(ImmediateType::SLTIU)),

  InstPattern::new("??????? ????? ????? 000 ????? 00000 11", Instruction::Immediate(ImmediateType::LB)),
  InstPattern::new("??????? ????? ????? 001 ????? 00000 11", Instruction::Immediate(ImmediateType::LH)),
  InstPattern::new("??????? ????? ????? 010 ????? 00000 11", Instruction::Immediate(ImmediateType::LW)),
  InstPattern::new("??????? ????? ????? 100 ????? 00000 11", Instruction::Immediate(ImmediateType::LBU)),
  InstPattern::new("??????? ????? ????? 101 ????? 00000 11", Instruction::Immediate(ImmediateType::LHU)),

  InstPattern::new("??????? ????? ????? 000 ????? 11001 11", Instruction::Immediate(ImmediateType::JALR)),

  InstPattern::new("0000000 00001 00000 000 00000 11100 11", Instruction::Immediate(ImmediateType::EBREAK)),
  InstPattern::new("0000000 00000 00000 000 00000 11100 11", Instruction::Immediate(ImmediateType::ECALL)),
  InstPattern::new("0000??? ????? 00000 000 00000 00011 11", Instruction::Immediate(ImmediateType::EBREAK)),

  InstPattern::new("??????? ????? ????? 001 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRW)),
  InstPattern::new("??????? ????? ????? 010 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRS)),
  InstPattern::new("??????? ????? ????? 011 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRC)),
  InstPattern::new("??????? ????? ????? 101 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRWI)),
  InstPattern::new("??????? ????? ????? 110 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRSI)),
  InstPattern::new("??????? ????? ????? 111 ????? 11100 11", Instruction::Immediate(ImmediateType::CSRRCI)),
  // rv64I
  InstPattern::new("??????? ????? ????? 110 ????? 00000 11", Instruction::Immediate(ImmediateType::LWU)),
  InstPattern::new("??????? ????? ????? 011 ????? 00000 11", Instruction::Immediate(ImmediateType::LD)),
  InstPattern::new("??????? ????? ????? 000 ????? 00110 11", Instruction::Immediate(ImmediateType::ADDIW)),
  InstPattern::new("000000? ????? ????? 001 ????? 00110 11", Instruction::Immediate(ImmediateType::SLLIW)),
  InstPattern::new("000000? ????? ????? 101 ????? 00110 11", Instruction::Immediate(ImmediateType::SRLIW)),
  InstPattern::new("010000? ????? ????? 101 ????? 00110 11", Instruction::Immediate(ImmediateType::SRAIW)),

  // Store
  // rv32I
  InstPattern::new("??????? ????? ????? 000 ????? 01000 11", Instruction::Store(StoreType::SB)),
  InstPattern::new("??????? ????? ????? 001 ????? 01000 11", Instruction::Store(StoreType::SH)),
  InstPattern::new("??????? ????? ????? 010 ????? 01000 11", Instruction::Store(StoreType::SW)),
  // rv64I
  InstPattern::new("??????? ????? ????? 011 ????? 01000 11", Instruction::Store(StoreType::SD)),
  // Branch
  // rv32I
  InstPattern::new("??????? ????? ????? 000 ????? 11000 11", Instruction::Branch(BranchType::BEQ)),
  InstPattern::new("??????? ????? ????? 001 ????? 11000 11", Instruction::Branch(BranchType::BNE)),
  InstPattern::new("??????? ????? ????? 100 ????? 11000 11", Instruction::Branch(BranchType::BLT)),
  InstPattern::new("??????? ????? ????? 101 ????? 11000 11", Instruction::Branch(BranchType::BGE)),
  InstPattern::new("??????? ????? ????? 110 ????? 11000 11", Instruction::Branch(BranchType::BLTU)),
  InstPattern::new("??????? ????? ????? 111 ????? 11000 11", Instruction::Branch(BranchType::BGEU)),
  // Jump
  // rv32I
  InstPattern::new("??????? ????? ????? ??? ????? 11011 11", Instruction::Jump(JumpType::JAL)),
  // Upper
  // rv32I
  InstPattern::new("??????? ????? ????? ??? ????? 01101 11", Instruction::Upper(UpperType::LUI)),
  InstPattern::new("??????? ????? ????? ??? ????? 00101 11", Instruction::Upper(UpperType::AUIPC)),]
}

#[rustfmt::skip]
/// Decode operands from instruction
pub fn disassemble(inst: &Inst, pc: u64) -> String {
  let (rd, rs1, rs2, imm) = (inst.rd, inst.rs1, inst.rs2, inst.imm);
  let gpr = vec!["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
  match inst.typ {
    Instruction::Register(RegisterType::ADD)  => format!("add   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::ADDW) => format!("addw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SUB)  => format!("sub   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SUBW) => format!("subw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::XOR)  => format!("xor   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::OR)   => format!("or    {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::AND)  => format!("and   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SLL)  => format!("sll   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SLLW) => format!("sllw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SRL)  => format!("srl   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SRA)  => format!("sra   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SRLW) => format!("srlw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SRAW) => format!("sraw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SLT)  => format!("slt   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::SLTU) => format!("sltu  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),

    Instruction::Immediate(ImmediateType::ADDI)  => format!("addi  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::ADDIW) => format!("addiw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::XORI)  => format!("xori  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::ORI)   => format!("ori   {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::ANDI)  => format!("andi  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SLLI)  => format!("slli  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SLLIW) => format!("slliw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SRLI)  => format!("srli  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SRLIW) => format!("srliw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SRAI)  => format!("srai  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SRAIW) => format!("sraiw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SLTI)  => format!("slti  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Instruction::Immediate(ImmediateType::SLTIU) => format!("sltiu {}, {}, {}", gpr[rd], gpr[rs1], imm),

    Instruction::Immediate(ImmediateType::LB)  => format!("lb    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Instruction::Immediate(ImmediateType::LBU) => format!("lbu   {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Instruction::Immediate(ImmediateType::LH)  => format!("lh    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Instruction::Immediate(ImmediateType::LHU) => format!("lhu   {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Instruction::Immediate(ImmediateType::LW)  => format!("lw    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Instruction::Immediate(ImmediateType::LWU) => format!("lwu   {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Instruction::Immediate(ImmediateType::LD)  => format!("ld    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),

    Instruction::Store(StoreType::SB) => format!("sb    {}, {}({})", gpr[rs2], imm, gpr[rs1]),
    Instruction::Store(StoreType::SH) => format!("sh    {}, {}({})", gpr[rs2], imm, gpr[rs1]),
    Instruction::Store(StoreType::SW) => format!("sw    {}, {}({})", gpr[rs2], imm, gpr[rs1]),
    Instruction::Store(StoreType::SD) => format!("sd    {}, {}({})", gpr[rs2], imm, gpr[rs1]),

    Instruction::Branch(BranchType::BEQ)  => format!("beq   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Instruction::Branch(BranchType::BNE)  => format!("bne   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Instruction::Branch(BranchType::BLT)  => format!("blt   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Instruction::Branch(BranchType::BGE)  => format!("bge   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Instruction::Branch(BranchType::BLTU) => format!("bltu  {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Instruction::Branch(BranchType::BGEU) => format!("bgeu  {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),

    Instruction::Jump(JumpType::JAL)            => format!("jal   {}, {:x}", gpr[rd], (imm as u64).wrapping_add(pc)),
    Instruction::Immediate(ImmediateType::JALR) => format!("jalr  {}, {}, {:x}", gpr[rd], gpr[rs1], imm),

    Instruction::Upper(UpperType::LUI)   => format!("lui   {}, {:x}", gpr[rd], imm),
    Instruction::Upper(UpperType::AUIPC) => format!("auipc {}, {:x}", gpr[rd], (pc as i64).wrapping_add(imm)),

    Instruction::Register(RegisterType::MUL)    => format!("mul      {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::MULH)   => format!("mulh     {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::MULHU)  => format!("mulhu    {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::MULHSU) => format!("mulhsu   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::MULW)   => format!("mulw     {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::DIV)    => format!("div      {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::DIVU)   => format!("divu     {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::DIVUW)  => format!("divuw    {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::DIVW)   => format!("divw     {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::REM)    => format!("rem      {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::REMU)   => format!("remu     {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::REMUW)  => format!("remuw    {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Instruction::Register(RegisterType::REMW)   => format!("remw     {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),

    Instruction::Register(RegisterType::MRET)   => format!("mret"),

    Instruction::Immediate(ImmediateType::ECALL)  => format!("ecall"),
    Instruction::Immediate(ImmediateType::EBREAK) => format!("ebreak"),

    Instruction::Immediate(ImmediateType::FENCE) => format!("fence"),

    Instruction::Immediate(ImmediateType::CSRRW)  => format!("csrrw  {}, {}", gpr[rd], gpr[rs1]),
    Instruction::Immediate(ImmediateType::CSRRS)  => format!("csrrs  {}, {}", gpr[rd], gpr[rs1]),
    Instruction::Immediate(ImmediateType::CSRRC)  => format!("csrrc  {}, {}", gpr[rd], gpr[rs1]),
    Instruction::Immediate(ImmediateType::CSRRWI) => format!("csrrwi {}, 0x{:x}, {}", gpr[rd], imm, rs1),
    Instruction::Immediate(ImmediateType::CSRRSI) => format!("csrrsi {}, 0x{:x}, {}", gpr[rd], imm, rs1),
    Instruction::Immediate(ImmediateType::CSRRCI) => format!("csrrci {}, 0x{:x}, {}", gpr[rd], imm, rs1),

    _ => format!("Unknown instruction")
  }
}