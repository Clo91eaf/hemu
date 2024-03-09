use crate::cpu::instruction::Instruction as Inst;
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
pub fn bits(x: u32, lo: u32, hi: u32) -> usize {
  assert!(hi >= lo);
  ((x >> lo) & bitmask(hi - lo)) as usize
}

pub fn sext(x: usize, len: u32) -> u64 {
  assert!(len <= 64);
  let extend_bits = 64 - len;
  (((x as i64) << extend_bits) >> extend_bits) as u64
}

#[rustfmt::skip]
pub fn decode_operand(inst: u32, inst_type: Inst) -> Option<(usize, usize, usize, i64)> {
  let (rd, rs1, rs2) = (bits(inst, 7, 12), bits(inst, 15, 20), bits(inst, 20, 25));
  match inst_type {
    Inst::Register(_) => Some((rd, rs1, rs2, 0)),
    Inst::Immediate(_) => Some((rd, rs1, 0, 
      sext(bits(inst, 20, 32), 12) as i64)),
    Inst::Store(_) => Some((0, rs1, rs2, 
      sext(bits(inst, 25, 32) << 5 | bits(inst, 7, 12), 12) as i64)),
    Inst::Branch(_) => Some((0, rs1, rs2, 
      sext(bits(inst, 31, 32) << 12 | bits(inst, 7, 8) << 11 | bits(inst, 25, 31) << 5 | bits(inst, 8, 12) << 1, 13) as i64)),
    Inst::Upper(_) => Some((rd, 0, 0, 
      sext(bits(inst, 12, 32) << 12, 32) as i64)),
    Inst::Jump(_) => Some((rd, 0, 0, 
      sext(bits(inst, 31, 32) << 20 | bits(inst, 21, 31) << 1 | bits(inst, 20, 21) << 11 | bits(inst, 12, 20) << 12, 21,) as i64)),
    Inst::ERROR => {None}
  }
}

#[rustfmt::skip]
/// Decode operands from instruction
pub fn disassemble(inst: u32, inst_type: Inst, pc: u64) -> String {
  let (rd, rs1, rs2, imm) = decode_operand(inst, inst_type).unwrap();
  let gpr = vec!["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
  match inst_type {
    Inst::Register(RegisterType::ADD)  => format!("add   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::ADDW) => format!("addw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SUB)  => format!("sub   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SUBW) => format!("subw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::XOR)  => format!("xor   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::OR)   => format!("or    {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::AND)  => format!("and   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SLL)  => format!("sll   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SLLW) => format!("sllw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SRL)  => format!("srl   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SRAW) => format!("sraw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SLT)  => format!("slt   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::SLTU) => format!("sltu  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),

    Inst::Immediate(ImmediateType::ADDI)  => format!("addi  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::ADDIW) => format!("addiw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::XORI)  => format!("xori  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::ORI)   => format!("ori   {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::ANDI)  => format!("andi  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SLLI)  => format!("slli  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SLLIW) => format!("slliw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SRLI)  => format!("srli  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SRLIW) => format!("srliw {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SRAI)  => format!("srai  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SLTI)  => format!("slti  {}, {}, {}", gpr[rd], gpr[rs1], imm),
    Inst::Immediate(ImmediateType::SLTIU) => format!("sltiu {}, {}, {}", gpr[rd], gpr[rs1], imm),

    Inst::Immediate(ImmediateType::LB)  => format!("lb    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Inst::Immediate(ImmediateType::LBU) => format!("lbu   {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Inst::Immediate(ImmediateType::LH)  => format!("lh    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Inst::Immediate(ImmediateType::LHU) => format!("lhu   {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Inst::Immediate(ImmediateType::LW)  => format!("lw    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Inst::Immediate(ImmediateType::LWU) => format!("lwu   {}, {:x}({})", gpr[rd], imm, gpr[rs1]),
    Inst::Immediate(ImmediateType::LD)  => format!("ld    {}, {:x}({})", gpr[rd], imm, gpr[rs1]),

    Inst::Store(StoreType::SB) => format!("sb    {}, {}({})", gpr[rs2], imm, gpr[rs1]),
    Inst::Store(StoreType::SH) => format!("sh    {}, {}({})", gpr[rs2], imm, gpr[rs1]),
    Inst::Store(StoreType::SW) => format!("sw    {}, {}({})", gpr[rs2], imm, gpr[rs1]),
    Inst::Store(StoreType::SD) => format!("sd    {}, {}({})", gpr[rs2], imm, gpr[rs1]),

    Inst::Branch(BranchType::BEQ)  => format!("beq   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Inst::Branch(BranchType::BNE)  => format!("bne   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Inst::Branch(BranchType::BLT)  => format!("blt   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Inst::Branch(BranchType::BGE)  => format!("bge   {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Inst::Branch(BranchType::BLTU) => format!("bltu  {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),
    Inst::Branch(BranchType::BGEU) => format!("bgeu  {}, {}, {:x}", gpr[rs1], gpr[rs2], (pc as i64).wrapping_add(imm)),

    Inst::Jump(JumpType::JAL)            => format!("jal   {}, {:x}", gpr[rd], (imm as u64).wrapping_add(pc)),
    Inst::Immediate(ImmediateType::JALR) => format!("jalr  {}, {}, {:x}", gpr[rd], gpr[rs1], imm),

    Inst::Upper(UpperType::LUI)   => format!("lui   {}, {:x}", gpr[rd], imm),
    Inst::Upper(UpperType::AUIPC) => format!("auipc {}, {:x}", gpr[rd], (pc as i64).wrapping_add(imm)),

    Inst::Register(RegisterType::MUL)  => format!("mul   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::MULW) => format!("mulw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::DIV)  => format!("div   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::DIVU) => format!("divu  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::DIVW) => format!("divw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::REM)  => format!("rem   {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::REMU) => format!("remu  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),
    Inst::Register(RegisterType::REMW) => format!("remw  {}, {}, {}", gpr[rd], gpr[rs1], gpr[rs2]),

    Inst::Immediate(ImmediateType::ECALL)  => format!("ecall"),
    Inst::Immediate(ImmediateType::EBREAK) => format!("ebreak"),

    _ => format!("Unknown instruction")
  }
}
