use crate::cpu::instruction::Inst;
use crate::cpu::instruction::*;

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
