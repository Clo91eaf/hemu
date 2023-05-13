mod cpu;
mod instruction;

use instruction::Instruction;

pub struct CpuState {
  gpr: [usize; 32],
  pc: usize,
}

struct Decode {
  pc: usize,
  snpc: usize,
  dnpc: usize,
  inst: u32,
}

struct InstPattern {
  pattern: &'static str,
  itype: Instruction,
}

impl InstPattern {
  fn new(pattern: &'static str, itype: Instruction) -> InstPattern {
    InstPattern { pattern, itype }
  }
}

lazy_static! {
  static ref CPU: CpuState = CpuState {
    gpr: [0; 32],
    pc: 0x80000000,
  };
}

fn match_inst(inst: u32, pattern: &str) -> bool {
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

fn bits(inst: u32, start: usize, end: usize) -> u32 {
  let (start, end) = (31 - start, 31 - end);
  (inst >> end) & ((1 << (start - end + 1)) - 1)
}

fn decode_operand(s: Decode, inst: Instruction) -> (i64, i64, i64, i64) {
  let (rd, rs1, rs2) = (
    bits(s.inst, 7, 12),
    bits(s.inst, 15, 20),
    bits(s.inst, 20, 25),
  );
  match inst {
    Instruction::Register(_) => (rd, rs1, rs2, 0),
    Instruction::Immediate(_) => (rd, rs1, 0, bits(s.inst, 20, 32)),
    Instruction::Store(_) => {
      (0, rs1, rs2, bits(s.inst, 25, 32) << 5 | bits(s.inst, 7, 12))
    }
    Instruction::Branch(_) => (
      0,
      rs1,
      rs2,
      bits(s.inst, 31, 32) << 12
        | bits(s.inst, 7, 8) << 11
        | bits(s.inst, 25, 31) << 5
        | bits(s.inst, 8, 12) << 1,
    ),
    Instruction::Upper(_) => (rd, 0, 0, bits(s.inst, 12, 32) << 12),
    Instruction::Jump(_) => (
      rs,
      0,
      0,
      bits(s.inst, 31, 32) << 20
        | bits(s.inst, 21, 31) << 1
        | bits(s.inst, 20, 21) << 11
        | bits(s.inst, 12, 20) << 12,
    ),
  }
}

fn fetch(s: Decode) -> u32 {
  let inst = unsafe { std::slice::from_raw_parts(s.pc as *const u8, 4) };
  let inst = u32::from_le_bytes([inst[0], inst[1], inst[2], inst[3]]);
  s.inst = inst;
  s.dnpc = s.pc + 4;
  inst
}

fn decode(inst: u32) -> Instruction {
  let partterns = [InstPattern::new(
    "??????? ????? ????? ??? ????? 00101 11",
    Instruction::Upper::AUIPC,
  )];
  for pattern in patterns.iter() {
    if match_inst(inst, pattern.pattern) {
      return pattern.itype;
    }
  }
}

fn execute(s: Decode, inst: Instruction) -> u32 {
  let (rd, rs1, rs2, imm) = decode_operand(s, inst);
  match inst {
    Instruction::Upper::AUIPC => {
      s.gpr[rd as usize] = s.pc + imm;
    }
  }
}

fn exec_once(s: Decode, pc: usize) {
  s.pc = pc;
  s.snpc = pc;
  // fetch stage
  s.inst = fetch(s);
  // decode stage
  let inst_type = decode(s.inst);
  // execute stage
  let res = execute(s);

  cpu.pc = s.dnpc;
}
