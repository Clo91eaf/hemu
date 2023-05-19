use crate::cpu::instruction::Instruction;
use crate::cpu::Decode;

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

pub fn bitmask(bits: u32) -> u32 {
  (1u32 << bits) - 1
}

// [lo, hi)
pub fn bits(x: u32, lo: u32, hi: u32) -> usize {
  assert!(hi >= lo);
  ((x >> lo) & bitmask(hi - lo)) as usize
}

pub fn sext(x: usize, len: u32) -> i64 {
  assert!(len <= 64);
  let extend_bits = 64 - len;
  ((x as i64) << extend_bits) >> extend_bits
}

pub fn decode_operand(s: &Decode, inst: Instruction) -> (usize, usize, usize, i64) {
  let (rd, rs1, rs2) = (
    bits(s.inst, 7, 12),
    bits(s.inst, 15, 20),
    bits(s.inst, 20, 25),
  );
  match inst {
    Instruction::Register(_) => (rd, rs1, rs2, 0),
    Instruction::Immediate(_) => (rd, rs1, 0, sext(bits(s.inst, 20, 32), 12)),
    Instruction::Store(_) => (
      0,
      rs1,
      rs2,
      sext(bits(s.inst, 25, 32) << 5 | bits(s.inst, 7, 12), 12),
    ),
    Instruction::Branch(_) => (
      0,
      rs1,
      rs2,
      sext(
        bits(s.inst, 31, 32) << 12
          | bits(s.inst, 7, 8) << 11
          | bits(s.inst, 25, 31) << 5
          | bits(s.inst, 8, 12) << 1,
        13,
      ),
    ),
    Instruction::Upper(_) => (rd, 0, 0, sext(bits(s.inst, 12, 32) << 12, 32)),
    Instruction::Jump(_) => (
      rd,
      0,
      0,
      sext(
        bits(s.inst, 31, 32) << 20
          | bits(s.inst, 21, 31) << 1
          | bits(s.inst, 20, 21) << 11
          | bits(s.inst, 12, 20) << 12,
        21,
      ),
    ),
  }
}