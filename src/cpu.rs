mod instruction;
pub mod memory;
mod statistic;
mod utils;

use ansi_term::Colour::{Green, Red};
use instruction::BranchType as B;
use instruction::ImmediateType as I;
use instruction::InstPattern as IP;
use instruction::Instruction as Inst;
use instruction::JumpType as J;
use instruction::RegisterType as R;
use instruction::StoreType as S;
use instruction::UpperType as U;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use memory::{read_data, read_inst, write_data};
use utils::{bits, decode_operand, match_inst, sext};

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
  pub pc: u64,
  snpc: u64,
  dnpc: u64,
  inst: u32,
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
      pc: 0x80000000,
      snpc: 0x80000000,
      dnpc: 0x80000000,
      inst: 0,
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
    self.inst = read_inst(self.snpc) as u32;
    self.snpc += 4;
  }

  pub fn decode(&self, inst_type: &mut Inst) {
    #[rustfmt::skip]
    let patterns = [
  // RegisterType
  // rv32I 
  IP::new("0000000 ????? ????? 000 ????? 01100 11", Inst::Register(R::ADD)),
  IP::new("0100000 ????? ????? 000 ????? 01100 11", Inst::Register(R::SUB)),
  IP::new("0000000 ????? ????? 100 ????? 01100 11", Inst::Register(R::XOR)),
  IP::new("0000000 ????? ????? 110 ????? 01100 11", Inst::Register(R::OR)),
  IP::new("0000000 ????? ????? 111 ????? 01100 11", Inst::Register(R::AND)),
  IP::new("0000000 ????? ????? 001 ????? 01100 11", Inst::Register(R::SLL)),
  IP::new("0000000 ????? ????? 101 ????? 01100 11", Inst::Register(R::SRL)),
  IP::new("0100000 ????? ????? 101 ????? 01100 11", Inst::Register(R::SRA)),
  IP::new("0000000 ????? ????? 010 ????? 01100 11", Inst::Register(R::SLT)),
  IP::new("0000000 ????? ????? 011 ????? 01100 11", Inst::Register(R::SLTU)),
  // rv32M
  IP::new("0000001 ????? ????? 000 ????? 01100 11", Inst::Register(R::MUL)),
  IP::new("0000001 ????? ????? 001 ????? 01100 11", Inst::Register(R::MULH)),
  IP::new("0000001 ????? ????? 010 ????? 01100 11", Inst::Register(R::MULHSU)),
  IP::new("0000001 ????? ????? 011 ????? 01100 11", Inst::Register(R::MULHU)),
  IP::new("0000001 ????? ????? 100 ????? 01100 11", Inst::Register(R::DIV)),
  IP::new("0000001 ????? ????? 101 ????? 01100 11", Inst::Register(R::DIVU)),
  IP::new("0000001 ????? ????? 110 ????? 01100 11", Inst::Register(R::REM)),
  IP::new("0000001 ????? ????? 111 ????? 01100 11", Inst::Register(R::REMU)),
  // rv64I
  IP::new("0000000 ????? ????? 000 ????? 01110 11", Inst::Register(R::ADDW)),
  IP::new("0100000 ????? ????? 000 ????? 01110 11", Inst::Register(R::SUBW)),
  IP::new("0000000 ????? ????? 001 ????? 01110 11", Inst::Register(R::SLLW)),
  IP::new("0000000 ????? ????? 101 ????? 01110 11", Inst::Register(R::SRLW)),
  IP::new("0100000 ????? ????? 101 ????? 01110 11", Inst::Register(R::SRAW)),
  // rv64M
  IP::new("0000001 ????? ????? 000 ????? 01110 11", Inst::Register(R::MULW)),
  IP::new("0000001 ????? ????? 100 ????? 01110 11", Inst::Register(R::DIVW)),
  IP::new("0000001 ????? ????? 100 ????? 01110 11", Inst::Register(R::DIVUW)),
  IP::new("0000001 ????? ????? 110 ????? 01110 11", Inst::Register(R::REMW)),
  IP::new("0000001 ????? ????? 110 ????? 01110 11", Inst::Register(R::REMUW)),

  // Immediate
  // rv32I
  IP::new("??????? ????? ????? 000 ????? 00100 11", Inst::Immediate(I::ADDI)),
  IP::new("??????? ????? ????? 100 ????? 00100 11", Inst::Immediate(I::XORI)),
  IP::new("??????? ????? ????? 110 ????? 00100 11", Inst::Immediate(I::ORI)),
  IP::new("??????? ????? ????? 111 ????? 00100 11", Inst::Immediate(I::ANDI)),
  IP::new("000000? ????? ????? 001 ????? 00100 11", Inst::Immediate(I::SLLI)),
  IP::new("000000? ????? ????? 101 ????? 00100 11", Inst::Immediate(I::SRLI)),
  IP::new("010000? ????? ????? 101 ????? 00100 11", Inst::Immediate(I::SRAI)),
  IP::new("??????? ????? ????? 010 ????? 00100 11", Inst::Immediate(I::SLTI)),
  IP::new("??????? ????? ????? 011 ????? 00100 11", Inst::Immediate(I::SLTIU)),

  IP::new("??????? ????? ????? 000 ????? 00000 11", Inst::Immediate(I::LB)),
  IP::new("??????? ????? ????? 001 ????? 00000 11", Inst::Immediate(I::LH)),
  IP::new("??????? ????? ????? 010 ????? 00000 11", Inst::Immediate(I::LW)),
  IP::new("??????? ????? ????? 100 ????? 00000 11", Inst::Immediate(I::LBU)),
  IP::new("??????? ????? ????? 101 ????? 00000 11", Inst::Immediate(I::LHU)),

  IP::new("??????? ????? ????? 000 ????? 11001 11", Inst::Immediate(I::JALR)),

  IP::new("0000000 00001 00000 000 00000 11100 11", Inst::Immediate(I::EBREAK)),
  IP::new("0000000 00000 00000 000 00000 11100 11", Inst::Immediate(I::ECALL)),
  // rv64I
  IP::new("??????? ????? ????? 110 ????? 00000 11", Inst::Immediate(I::LWU)),
  IP::new("??????? ????? ????? 011 ????? 00000 11", Inst::Immediate(I::LD)),
  IP::new("??????? ????? ????? 000 ????? 00110 11", Inst::Immediate(I::ADDIW)),
  IP::new("000000? ????? ????? 001 ????? 00110 11", Inst::Immediate(I::SLLIW)),
  IP::new("000000? ????? ????? 101 ????? 00110 11", Inst::Immediate(I::SRLIW)),
  IP::new("010000? ????? ????? 101 ????? 00110 11", Inst::Immediate(I::SRAIW)),
  // Store
  // rv32I
  IP::new("??????? ????? ????? 000 ????? 01000 11", Inst::Store(S::SB)),
  IP::new("??????? ????? ????? 001 ????? 01000 11", Inst::Store(S::SH)),
  IP::new("??????? ????? ????? 010 ????? 01000 11", Inst::Store(S::SW)),
  // rv64I
  IP::new("??????? ????? ????? 011 ????? 01000 11", Inst::Store(S::SD)),
  // Branch
  // rv32I
  IP::new("??????? ????? ????? 000 ????? 11000 11", Inst::Branch(B::BEQ)),
  IP::new("??????? ????? ????? 001 ????? 11000 11", Inst::Branch(B::BNE)),
  IP::new("??????? ????? ????? 100 ????? 11000 11", Inst::Branch(B::BLT)),
  IP::new("??????? ????? ????? 101 ????? 11000 11", Inst::Branch(B::BGE)),
  IP::new("??????? ????? ????? 110 ????? 11000 11", Inst::Branch(B::BLTU)),
  IP::new("??????? ????? ????? 111 ????? 11000 11", Inst::Branch(B::BGEU)),
  // Jump
  // rv32I
  IP::new("??????? ????? ????? ??? ????? 11011 11", Inst::Jump(J::JAL)),
  // Upper
  // rv32I
  IP::new("??????? ????? ????? ??? ????? 01101 11", Inst::Upper(U::LUI)),
  IP::new("??????? ????? ????? ??? ????? 00101 11", Inst::Upper(U::AUIPC)),
    ];
    patterns
      .iter()
      .find(|pattern| match_inst(self.inst, pattern.pattern))
      .map(|pattern| *inst_type = pattern.itype);
  }

  #[rustfmt::skip]
  pub fn execute(&mut self, inst_type: Inst) {
    let (rd, rs1, rs2, imm) = decode_operand(self.inst, inst_type);
    self.dnpc = self.snpc;
    match inst_type {
      Inst::Register(R::ADD)  => {self.gpr[rd] = (self.gpr[rs1] as i64).wrapping_add(self.gpr[rs2] as i64) as u64;}
      Inst::Register(R::ADDW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64).wrapping_add(self.gpr[rs2] as i64) as usize, 32);}
      Inst::Register(R::SUB)  => {self.gpr[rd] = (self.gpr[rs1] as i64).wrapping_sub(self.gpr[rs2] as i64) as u64;}
      Inst::Register(R::SUBW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64).wrapping_sub(self.gpr[rs2] as i64) as usize, 32);}
      Inst::Register(R::XOR)  => {self.gpr[rd] = self.gpr[rs1] ^ self.gpr[rs2];}
      Inst::Register(R::OR)   => {self.gpr[rd] = self.gpr[rs1] | self.gpr[rs2];}
      Inst::Register(R::AND)  => {self.gpr[rd] = self.gpr[rs1] & self.gpr[rs2];}
      Inst::Register(R::SLL)  => {self.gpr[rd] = self.gpr[rs1] << self.gpr[rs2];}
      Inst::Register(R::SLLW) => {self.gpr[rd] = sext((self.gpr[rs1] << self.gpr[rs2]) as usize, 32);}
      Inst::Register(R::SRL)  => {self.gpr[rd] = self.gpr[rs1] >> self.gpr[rs2];}
      Inst::Register(R::SRA)  => {self.gpr[rd] = ((self.gpr[rs1] as i64) >> self.gpr[rs2]) as u64;}
      Inst::Register(R::SRLW) => {self.gpr[rd] = sext((self.gpr[rs1] as u32 >> bits(self.gpr[rs2] as u32, 0, 5)) as usize, 32);}
      Inst::Register(R::SRAW) => {self.gpr[rd] = sext((self.gpr[rs1] as i32 >> bits(self.gpr[rs2] as u32, 0, 5)) as usize, 32);}
      Inst::Register(R::SLT)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {1} else {0};}
      Inst::Register(R::SLTU) => {self.gpr[rd] = if self.gpr[rs1] < self.gpr[rs2] {1} else {0};}

      Inst::Immediate(I::ADDI)  => {self.gpr[rd] = (self.gpr[rs1] as i64).wrapping_add(imm) as u64;}
      Inst::Immediate(I::ADDIW) => {self.gpr[rd] = sext((self.gpr[rs1] as i64).wrapping_add(imm) as usize, 32);}
      Inst::Immediate(I::XORI)  => {self.gpr[rd] = self.gpr[rs1] ^ imm as u64;}
      Inst::Immediate(I::ORI)   => {self.gpr[rd] = self.gpr[rs1] | imm as u64;}
      Inst::Immediate(I::ANDI)  => {self.gpr[rd] = self.gpr[rs1] & imm as u64;}
      Inst::Immediate(I::SLLI)  => {self.gpr[rd] = self.gpr[rs1] << bits(imm as u32, 0, 6);}
      Inst::Immediate(I::SLLIW) => {self.gpr[rd] = sext((self.gpr[rs1] << bits(imm as u32, 0, 6)) as usize, 32);}
      Inst::Immediate(I::SRLI)  => {self.gpr[rd] = self.gpr[rs1] >> bits(imm as u32, 0, 6);}
      Inst::Immediate(I::SRLIW) => {self.gpr[rd] = sext((self.gpr[rs1] as u32 >> bits(imm as u32, 0, 6)) as usize, 32);}
      Inst::Immediate(I::SRAI)  => {self.gpr[rd] = (self.gpr[rs1] as i64 >> bits(imm as u32, 0, 6)) as u64;}
      Inst::Immediate(I::SRAIW) => {self.gpr[rd] = sext((self.gpr[rs1] as i32 >> bits(imm as u32, 0, 6)) as usize, 32);}
      Inst::Immediate(I::SLTI)  => {self.gpr[rd] = if (self.gpr[rs1] as i64) < imm {1} else {0};}
      Inst::Immediate(I::SLTIU) => {self.gpr[rd] = if self.gpr[rs1] < imm as u64 {1} else {0};}

      Inst::Immediate(I::LB)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 1) as usize, 8);}
      Inst::Immediate(I::LBU) => {self.gpr[rd] = read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 1);}
      Inst::Immediate(I::LH)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 2) as usize, 16);}
      Inst::Immediate(I::LHU) => {self.gpr[rd] = read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 2);}
      Inst::Immediate(I::LW)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 4) as usize, 32);}
      Inst::Immediate(I::LWU) => {self.gpr[rd] = read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 4);}
      Inst::Immediate(I::LD)  => {self.gpr[rd] = sext(read_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 8) as usize, 64);}

      Inst::Store(S::SB) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 1, self.gpr[rs2]);}
      Inst::Store(S::SH) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 2, self.gpr[rs2]);}
      Inst::Store(S::SW) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 4, self.gpr[rs2]);}
      Inst::Store(S::SD) => {write_data((self.gpr[rs1] as i64).wrapping_add(imm) as u64, 8, self.gpr[rs2]);}

      Inst::Branch(B::BEQ)  => {if self.gpr[rs1] == self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Inst::Branch(B::BNE)  => {if self.gpr[rs1] != self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Inst::Branch(B::BLT)  => {if (self.gpr[rs1] as i64) < (self.gpr[rs2] as i64) {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Inst::Branch(B::BGE)  => {if (self.gpr[rs1] as i64) >= (self.gpr[rs2] as i64) {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Inst::Branch(B::BLTU) => {if self.gpr[rs1] < self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}
      Inst::Branch(B::BGEU) => {if self.gpr[rs1] >= self.gpr[rs2] {self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}}

      Inst::Jump(J::JAL)       => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.pc as i64).wrapping_add(imm) as u64;}
      Inst::Immediate(I::JALR) => {self.gpr[rd] = self.pc + 4; self.dnpc = (self.gpr[rs1] as i64).wrapping_add(imm) as u64;}

      Inst::Upper(U::LUI)   => {self.gpr[rd] = imm as u64;}
      Inst::Upper(U::AUIPC) => {self.gpr[rd] = (self.pc as i64).wrapping_add(imm) as u64;}

      Inst::Register(R::MUL)    => {self.gpr[rd] = self.gpr[rs1].wrapping_mul(self.gpr[rs2]);}
      Inst::Register(R::MULH)   => {self.gpr[rd] = ((self.gpr[rs1] as i128 * self.gpr[rs2] as i128) >> 64) as u64;}
      Inst::Register(R::MULHU)  => {self.gpr[rd] = ((self.gpr[rs1] as u128 * self.gpr[rs2] as u128) >> 64) as u64;}
      Inst::Register(R::MULHSU) => {self.gpr[rd] = ((self.gpr[rs1] as i128 * (self.gpr[rs2] as u128) as i128) >> 64) as u64;}
      Inst::Register(R::MULW)   => {self.gpr[rd] = sext(self.gpr[rs1].wrapping_mul(self.gpr[rs2]) as usize, 32);}
      Inst::Register(R::DIV)    => {self.gpr[rd] = (self.gpr[rs1] as i32 / self.gpr[rs2] as i32) as u64;}
      Inst::Register(R::DIVU)   => {self.gpr[rd] = self.gpr[rs1] / self.gpr[rs2];}
      Inst::Register(R::DIVUW)  => {self.gpr[rd] = sext((self.gpr[rs1] as u32 / self.gpr[rs2] as u32) as usize, 32);}
      Inst::Register(R::DIVW)   => {self.gpr[rd] = sext((self.gpr[rs1] as i32 / self.gpr[rs2] as i32) as usize, 32);}
      Inst::Register(R::REM)    => {self.gpr[rd] = (self.gpr[rs1] as i32 % self.gpr[rs2] as i32) as u64;}
      Inst::Register(R::REMU)   => {self.gpr[rd] = self.gpr[rs1] % self.gpr[rs2];}
      Inst::Register(R::REMUW)  => {self.gpr[rd] = sext((self.gpr[rs1] as u32 % self.gpr[rs2] as u32) as usize, 64);}
      Inst::Register(R::REMW)   => {self.gpr[rd] = sext((self.gpr[rs1] as i32 % self.gpr[rs2] as i32) as usize, 32);}

      Inst::Immediate(I::ECALL)  => {todo!();}
      Inst::Immediate(I::EBREAK) => {self.hemu_trap();}

      _ => {todo!("{:?} not implemented", inst_type);}
    }
    self.gpr[0] = 0;
  }

  fn exec_once(&mut self) {
    // pipeline start
    let mut inst_type = Inst::ERROR;
    self.snpc = self.pc;
    // fetch stage
    self.fetch();
    // decode stage
    self.decode(&mut inst_type);
    // execute stage (including memory stage and write back stage)
    self.execute(inst_type);
    // print disassemble
    log::info!(
      "{:08x}: {:08x}\t{}",
      self.pc,
      self.inst,
      utils::disassemble(self.inst, inst_type, self.pc)
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
          return 0;
        } else {
          log::error!("{}", Red.bold().paint("HIT BAD TRAP"));
          return -1;
        }
      }
      CpuState::Running => {}
      CpuState::Quit => {
        self.statistic();
      }
    }
    0
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
