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
    InstPattern {
      pattern,
      itype,
    }
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

fn fetch(s: Decode) -> u32 {
  let inst = unsafe { std::slice::from_raw_parts(s.pc as *const u8, 4) };
  let inst = u32::from_le_bytes([inst[0], inst[1], inst[2], inst[3]]);
  s.inst = inst;
  s.dnpc = s.pc + 4;
  inst
}

fn decode(inst: u32) -> Instruction {
  let partterns = [
    InstPattern::new("??????? ????? ????? ??? ????? 00101 11", Instruction::Upper::AUIPC),
  ];
  for pattern in patterns.iter() {
    if match_inst(inst, pattern.pattern) {
      return pattern.itype;
    }
  }
}

fn execute(s: Decode, inst: Instruction) -> u32 {
  match inst {
    Instruction::Upper::AUIPC => {
      let rd = (s.inst >> 7) & 0x1f;
      let imm = (s.inst >> 12) & 0xfffff;
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
