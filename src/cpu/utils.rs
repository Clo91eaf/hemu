use crate::cpu::instruction::*;

pub fn match_inst(inst: u32, pattern: &str) -> bool {
  let mut mask = 0;
  let mut expected = 0;

  for (i, c) in pattern.replace(" ", "").chars().enumerate() {
    if c == '0' || c == '1' {
      mask |= 1 << (31 - i);
      expected |= (c.to_digit(2).unwrap() as u32) << (31 - i);
    }
  }

  (inst & mask) == expected
}

fn bitmask(bits: u32) -> u32 {
  (1u32 << bits) - 1
}

// [lo, hi)
fn bits(x: u32, lo: u32, hi: u32) -> usize {
  assert!(hi >= lo);
  ((x >> lo) & bitmask(hi - lo)) as usize
}

pub fn sext(x: usize, len: u32) -> i64 {
  assert!(len <= 64);
  let extend_bits = 64 - len;
  ((x as i64) << extend_bits) >> extend_bits
}

pub fn decode_operand(
  inst: u32,
  inst_type: Instruction,
) -> (usize, usize, usize, i64) {
  let (rd, rs1, rs2) =
    (bits(inst, 7, 12), bits(inst, 15, 20), bits(inst, 20, 25));
  match inst_type {
    Instruction::Register(_) => (rd, rs1, rs2, 0),
    Instruction::Immediate(_) => (rd, rs1, 0, sext(bits(inst, 20, 32), 12)),
    Instruction::Store(_) => (
      0,
      rs1,
      rs2,
      sext(bits(inst, 25, 32) << 5 | bits(inst, 7, 12), 12),
    ),
    Instruction::Branch(_) => (
      0,
      rs1,
      rs2,
      sext(
        bits(inst, 31, 32) << 12
          | bits(inst, 7, 8) << 11
          | bits(inst, 25, 31) << 5
          | bits(inst, 8, 12) << 1,
        13,
      ),
    ),
    Instruction::Upper(_) => (rd, 0, 0, sext(bits(inst, 12, 32) << 12, 32)),
    Instruction::Jump(_) => (
      rd,
      0,
      0,
      sext(
        bits(inst, 31, 32) << 20
          | bits(inst, 21, 31) << 1
          | bits(inst, 20, 21) << 11
          | bits(inst, 12, 20) << 12,
        21,
      ),
    ),
  }
}

#[rustfmt::skip]
/// Decode operands from instruction
pub fn disassemble(inst: u32, inst_type: Instruction) -> String {
  let (rd, rs1, rs2, imm) = decode_operand(inst, inst_type);
  match inst_type {
    Instruction::Register(RegisterType::ADD)  => format!("add   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::ADDW) => format!("addw  {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::SUB)  => format!("sub   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::XOR)  => format!("xor   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::OR)   => format!("or    {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::AND)  => format!("and   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::SLL)  => format!("sll   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::SLLW) => format!("sllw  {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::SRL)  => format!("srl   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::SLT)  => format!("slt   {}, {}, {}", rd, rs1, rs2),
    Instruction::Register(RegisterType::SLTU) => format!("sltu  {}, {}, {}", rd, rs1, rs2),

    Instruction::Immediate(ImmediateType::ADDI)  => format!("addi  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::ADDIW) => format!("addiw {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::XORI)  => format!("xori  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::ORI)   => format!("ori   {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::ANDI)  => format!("andi  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::SLLI)  => format!("slli  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::SRLI)  => format!("srli  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::SRAI)  => format!("srai  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::SLTI)  => format!("slti  {}, {}, {}", rd, rs1, imm),
    Instruction::Immediate(ImmediateType::SLTIU) => format!("sltiu {}, {}, {}", rd, rs1, imm),

    Instruction::Immediate(ImmediateType::LB)  => format!("lb    {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LBU) => format!("lbu   {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LH)  => format!("lh    {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LHU) => format!("lhu   {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LW)  => format!("lw    {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LWU) => format!("lwu   {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LD)  => format!("ld    {}, {}({})", rd, imm, rs1),
    Instruction::Immediate(ImmediateType::LDU) => format!("ldu   {}, {}({})", rd, imm, rs1),

    Instruction::Store(StoreType::SB) => format!("sb    {}, {}({})", rs2, imm, rs1),
    Instruction::Store(StoreType::SH) => format!("sh    {}, {}({})", rs2, imm, rs1),
    Instruction::Store(StoreType::SW) => format!("sw    {}, {}({})", rs2, imm, rs1),
    Instruction::Store(StoreType::SD) => format!("sd    {}, {}({})", rs2, imm, rs1),

    Instruction::Branch(BranchType::BEQ)  => format!("beq   {}, {}, {}", rs1, rs2, imm),
    Instruction::Branch(BranchType::BNE)  => format!("bne   {}, {}, {}", rs1, rs2, imm),
    Instruction::Branch(BranchType::BLT)  => format!("blt   {}, {}, {}", rs1, rs2, imm),
    Instruction::Branch(BranchType::BGE)  => format!("bge   {}, {}, {}", rs1, rs2, imm),
    Instruction::Branch(BranchType::BLTU) => format!("bltu  {}, {}, {}", rs1, rs2, imm),
    Instruction::Branch(BranchType::BGEU) => format!("bgeu  {}, {}, {}", rs1, rs2, imm),

    Instruction::Jump(JumpType::JAL)            => format!("jal   {}, {}", rd, imm),
    Instruction::Immediate(ImmediateType::JALR) => format!("jalr  {}, {}, {}", rd, rs1, imm),

    Instruction::Upper(UpperType::LUI)   => format!("lui   {}, {}", rd, imm),
    Instruction::Upper(UpperType::AUIPC) => format!("auipc {}, {}", rd, imm),

    Instruction::Immediate(ImmediateType::ECALL)  => format!("ecall"),
    Instruction::Immediate(ImmediateType::EBREAK) => format!("ebreak"),

    _ => format!("Unknown instruction")
  }
}