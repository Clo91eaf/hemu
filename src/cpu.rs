mod csr;
mod instruction;
pub mod memory;
mod statistic;
mod utils;

use ansi_term::Colour::{Green, Red};
use csr::*;
use instruction::BranchType as B;
use instruction::ImmediateType as I;
use instruction::Instruction as Insn;
use instruction::Inst;
use instruction::JumpType as J;
use instruction::RegisterType as R;
use instruction::StoreType as S;
use instruction::UpperType as U;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use memory::{read_data, read_inst, write_data};
use utils::{bits, sext};

#[derive(PartialEq, Debug)]
pub enum CpuState {
  Running,
  // Stopped,
  Ended,
  // Aborted,
  Quit,
}

pub struct Halt {
  pc: u32,
  pub ret: u32,
}

impl Halt {
  pub fn new() -> Halt {
    Halt { pc: 0, ret: 0 }
  }
}

pub struct Cpu {
  pub gpr: [u64; 32],
  pub csr: Csr,
  pub pc: u64,
  snpc: u64,
  dnpc: u64,
  inst: Inst,
  pub state: CpuState,
  pub halt: Halt,
  statistic: statistic::Statistic,
  diff: Option<BufReader<std::fs::File>>,
}

impl Cpu {
  pub fn new(diff: Option<PathBuf>) -> Cpu {
    let diff = diff.map(|diff_file_path| {
      let file = std::fs::File::open(&diff_file_path).unwrap();
      BufReader::new(file)
    });

    Cpu {
      gpr: [0; 32],
      csr: Csr::new(),
      pc: 0x80000000,
      snpc: 0x80000000,
      dnpc: 0x80000000,
      inst: Inst::new(),
      state: CpuState::Running,
      halt: Halt::new(),
      statistic: statistic::Statistic::new(),
      diff,
    }
  }

  fn hemu_trap(&mut self) {
    self.state = CpuState::Ended;
    self.halt.pc = self.pc as u32;
    self.halt.ret = self.gpr[10] as u32;

    log::info!("hemu trap, pc = {:x}, ret = {}", self.pc, self.gpr[10]);
  }

  pub fn fetch(&mut self) {
    let bits = read_inst(self.snpc) as u32;
    self.inst.set_bits(bits);
    self.snpc += 4;
  }

  #[rustfmt::skip]
  pub fn execute(&mut self) {
    let (rd, rs1, rs2, imm) = (self.inst.rd, self.inst.rs1, self.inst.rs2, self.inst.imm);
    self.dnpc = self.snpc;
    let csr = bits(self.inst.bits, 20, 31) as u16;
    match self.inst.typ {
      Insn::Register(R::ADD)  => {self.gpr[rd] = (self.gpr[rs1] as i64).wrapping_add(self.gpr[rs2] as i64) as u64;}
      Insn::Register(R::ADDW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64).wrapping_add(self.gpr[rs2] as i64) as usize, 32);}
      Insn::Register(R::SUB)  => {self.gpr[rd] = (self.gpr[rs1] as i64).wrapping_sub(self.gpr[rs2] as i64) as u64;}
      Insn::Register(R::SUBW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64).wrapping_sub(self.gpr[rs2] as i64) as usize, 32);}
      Insn::Register(R::XOR)  => {self.gpr[rd] = self.gpr[rs1] ^ self.gpr[rs2];}
      Insn::Register(R::OR)   => {self.gpr[rd] = self.gpr[rs1] | self.gpr[rs2];}
      Insn::Register(R::AND)  => {self.gpr[rd] = self.gpr[rs1] & self.gpr[rs2];}
      Insn::Register(R::SLL)  => {self.gpr[rd] = self.gpr[rs1] << self.gpr[rs2];}
      Insn::Register(R::SLLW) => {self.gpr[rd] = sext((self.gpr[rs1] << self.gpr[rs2]) as usize, 32);}
      Insn::Register(R::SRL)  => {self.gpr[rd] = self.gpr[rs1] >> self.gpr[rs2];}
      Insn::Register(R::SRA)  => {self.gpr[rd] = ((self.gpr[rs1] as i64) >> self.gpr[rs2]) as u64;}
      Insn::Register(R::SRLW) => {self.gpr[rd] = sext((self.gpr[rs1] as u32 >> bits(self.gpr[rs2] as u32, 0, 5)) as usize, 32);}
      Insn::Register(R::SRAW) => {self.gpr[rd] = sext((self.gpr[rs1] as i32 >> bits(self.gpr[rs2] as u32, 0, 5)) as usize, 32);}
      Insn::Register(R::SLT)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {1} else {0};}
      Insn::Register(R::SLTU) => {self.gpr[rd] = if self.gpr[rs1] < self.gpr[rs2] {1} else {0};}

      Insn::Immediate(I::ADDI)  => {self.gpr[rd] = (self.gpr[rs1] as i64).wrapping_add(imm) as u64;}
      Insn::Immediate(I::ADDIW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64).wrapping_add(imm) as usize, 32);}
      Insn::Immediate(I::XORI)  => {self.gpr[rd] = self.gpr[rs1] ^ imm as u64;}
      Insn::Immediate(I::ORI)   => {self.gpr[rd] = self.gpr[rs1] | imm as u64;}
      Insn::Immediate(I::ANDI)  => {self.gpr[rd] = self.gpr[rs1] & imm as u64;}
      Insn::Immediate(I::SLLI)  => {self.gpr[rd] = self.gpr[rs1] << bits(imm as u32, 0, 6);}
      Insn::Immediate(I::SLLIW) => {self.gpr[rd] = sext((self.gpr[rs1] << bits(imm as u32, 0, 6)) as usize, 32);}
      Insn::Immediate(I::SRLI)  => {self.gpr[rd] = self.gpr[rs1] >> bits(imm as u32, 0, 6);}
      Insn::Immediate(I::SRLIW) => {self.gpr[rd] = sext((self.gpr[rs1] as u32 >> bits(imm as u32, 0, 6)) as usize, 32);}
      Insn::Immediate(I::SRAI)  => {self.gpr[rd] = (self.gpr[rs1] as i64 >> bits(imm as u32, 0, 6)) as u64;}
      Insn::Immediate(I::SRAIW) => {self.gpr[rd] = sext((self.gpr[rs1] as i32 >> bits(imm as u32, 0, 6)) as usize, 32);}
      Insn::Immediate(I::SLTI)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < imm {1} else {0};}
      Insn::Immediate(I::SLTIU) => {self.gpr[rd] = if self.gpr[rs1] < imm as u64 {1} else {0};}

      Insn::Immediate(I::LB)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 1) as usize, 8);}
      Insn::Immediate(I::LBU) => {self.gpr[rd] = read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 1);}
      Insn::Immediate(I::LH)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 2) as usize, 16);}
      Insn::Immediate(I::LHU) => {self.gpr[rd] = read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 2);}
      Insn::Immediate(I::LW)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 4) as usize, 32);}
      Insn::Immediate(I::LWU) => {self.gpr[rd] = read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 4);}
      Insn::Immediate(I::LD)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 8) as usize, 64);}

      Insn::Store(S::SB) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 1, self.gpr[rs2]);}
      Insn::Store(S::SH) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 2, self.gpr[rs2]);}
      Insn::Store(S::SW) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 4, self.gpr[rs2]);}
      Insn::Store(S::SD) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 8, self.gpr[rs2]);}

      Insn::Branch(B::BEQ)  => {if self.gpr[rs1] == self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Insn::Branch(B::BNE)  => {if self.gpr[rs1] != self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Insn::Branch(B::BLT)  => {if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Insn::Branch(B::BGE)  => {if (self.gpr[rs1] as i64) >= (self.gpr[rs2] as i64) {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Insn::Branch(B::BLTU) => {if self.gpr[rs1] < self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Insn::Branch(B::BGEU) => {if self.gpr[rs1] >= self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}

      Insn::Jump(J::JAL)       => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}
      Insn::Immediate(I::JALR) => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.gpr[rs1] as i64).wrapping_add(imm) as u64;}

      Insn::Upper(U::LUI)   => {self.gpr[rd] = imm as u64;}
      Insn::Upper(U::AUIPC) => {self.gpr[rd] = (self.pc as i64).wrapping_add(imm) as u64;}

      Insn::Register(R::MUL)    => {self.gpr[rd] = self.gpr[rs1].wrapping_mul(self.gpr[rs2]);}
      Insn::Register(R::MULH)   => {self.gpr[rd] = ((self.gpr[rs1] as i128 * self.gpr[rs2] as i128) >> 64) as u64;}
      Insn::Register(R::MULHU)  => {self.gpr[rd] = ((self.gpr[rs1] as u128 * self.gpr[rs2] as u128) >> 64) as u64;}
      Insn::Register(R::MULHSU) => {self.gpr[rd] = ((self.gpr[rs1] as i128 * (self.gpr[rs2] as u128) as i128) >> 64) as u64;}
      Insn::Register(R::MULW)   => {self.gpr[rd] = sext(self.gpr[rs1].wrapping_mul(self.gpr[rs2]) as usize, 32);}
      Insn::Register(R::DIV)    => {self.gpr[rd] = (self.gpr[rs1] as i32 / self.gpr[rs2] as i32) as u64;}
      Insn::Register(R::DIVU)   => {self.gpr[rd] = self.gpr[rs1] / self.gpr[rs2];}
      Insn::Register(R::DIVUW)  => {self.gpr[rd] = sext((self.gpr[rs1] as u32 / self.gpr[rs2] as u32) as usize, 32);}
      Insn::Register(R::DIVW)   => {self.gpr[rd] = sext((self.gpr[rs1] as i32 / self.gpr[rs2] as i32) as usize, 32);}
      Insn::Register(R::REM)    => {self.gpr[rd] = (self.gpr[rs1] as i32 % self.gpr[rs2] as i32) as u64;}
      Insn::Register(R::REMU)   => {self.gpr[rd] = self.gpr[rs1] % self.gpr[rs2];}
      Insn::Register(R::REMUW)  => {self.gpr[rd] = sext((self.gpr[rs1] as u32 % self.gpr[rs2] as u32) as usize, 64);}
      Insn::Register(R::REMW)   => {self.gpr[rd] = sext((self.gpr[rs1] as i32 % self.gpr[rs2] as i32) as usize, 32);}

      Insn::Register(R::MRET)   => {self.dnpc = self.csr.read(MEPC).wrapping_add(4);}

      Insn::Immediate(I::ECALL)  => {todo!();}
      Insn::Immediate(I::EBREAK) => {self.hemu_trap();}
      Insn::Immediate(I::FENCE)  => {}

      Insn::Immediate(I::CSRRW)  => {let t = self.csr.read(csr); self.csr.write(csr, self.gpr[rs1]); self.gpr[rd] = t;}
      Insn::Immediate(I::CSRRS)  => {let t = self.csr.read(csr); self.csr.write(csr, t | self.gpr[rs1]); self.gpr[rd] = t;}
      Insn::Immediate(I::CSRRC)  => {let t = self.csr.read(csr); self.csr.write(csr, t &!self.gpr[rs1]); self.gpr[rd] = t;}
      Insn::Immediate(I::CSRRWI) => {self.gpr[rd] = self.csr.read(csr); self.csr.write(csr, rs1 as u64);}
      Insn::Immediate(I::CSRRSI) => {let t = self.csr.read(csr); self.csr.write(csr, t | rs1 as u64); self.gpr[rd] = t;}
      Insn::Immediate(I::CSRRCI) => {let t = self.csr.read(csr); self.csr.write(csr, t & !(rs1 as u64)); self.gpr[rd] = t;}

      _ => {todo!("{:?} not implemented", self.inst.typ);}
    }
    self.gpr[0] = 0;
  }

  fn exec_once(&mut self) {
    // pipeline start
    self.snpc = self.pc;
    // fetch stage
    self.fetch();
    // execute stage (including memory stage and write back stage)
    self.execute();
    // print disassemble
    log::info!(
      "{:08x}: {:08x}\t{}",
      self.pc,
      &self.inst.bits,
      self.inst.disassemble(self.pc)
    );
    self.difftest().unwrap();
    // update pc
    self.pc = self.dnpc;
  }

  fn exec_ntimes(&mut self, n: usize) {
    for _ in 0..n {
      self.exec_once();
      self.statistic.inc_count();
      if self.state != CpuState::Running {
        break;
      }
    }
  }

  fn statistic(&self) {
    log::info!("host time spend = {:?}", self.statistic.time);
    log::info!("total guest instructions = {:?}", self.statistic.count);
    log::info!(
      "simulation frequency = {:?}",
      (self.statistic.count) as f64 / (self.statistic.time.as_secs_f64())
    );
  }

  pub fn exec(&mut self, n: usize) -> i32 {
    let start_time = self.statistic.start_timer();

    self.exec_ntimes(n);

    self.statistic.stop_timer(start_time);

    match self.state {
      CpuState::Ended => {
        if self.halt.ret == 0 {
          println!("{}", Green.bold().paint("HIT GOOD TRAP"));
          self.statistic();
          0
        } else {
          log::error!("{}", Red.bold().paint("HIT BAD TRAP"));
          -1
        }
      }
      CpuState::Running => 0,
      CpuState::Quit => {
        self.statistic();
        0
      }
    }
  }

  pub fn dump_registers(&self) {
    let gpr = vec![
      "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7",
      "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
    ];
    for i in 0..8 {
      let mut line = String::with_capacity(80);
      for j in 0..4 {
        let index = i * 4 + j;
        line.push_str(&format!("{:4}=0x{:08x}|", gpr[index], self.gpr[index]));
      }
      log::info!("{}", line);
    }
  }

  pub fn dump_watches(&self) {
    todo!("watch points not implemented!");
  }

  fn difftest(&mut self) -> anyhow::Result<()> {
    // get numbers from diff file
    let mut line = String::new();

    if let Some(diff) = self.diff.as_mut() {
      diff.read_line(&mut line)?;
    } else {
      return Ok(());
    }

    let numbers = line
      .split_whitespace()
      .map(|s| u64::from_str_radix(&s, 16).unwrap())
      .collect::<Vec<u64>>();

    // if numbers.len() == 0, there maybe a empty line
    if numbers.len() == 0 {
      return Ok(());
    }

    // difftest check
    if numbers[0] != self.pc {
      panic!("pc = {:x}, ref = {:x}", self.pc, numbers[0]);
    }
    (0..32).for_each(|i| {
      if numbers[i + 1] != self.gpr[i] {
        panic!("x{:02} = {:x}, ref = {:x}", i, self.gpr[i], numbers[i + 1]);
      }
    });

    Ok(())
  }
}
